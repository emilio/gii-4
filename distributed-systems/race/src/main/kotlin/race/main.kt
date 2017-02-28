package race

import org.springframework.boot.SpringApplication
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import org.springframework.web.bind.annotation.RequestParam
import java.net.HttpURLConnection
import java.net.URL
import java.util.concurrent.locks.Condition
import java.util.concurrent.locks.ReentrantLock
import com.fasterxml.jackson.module.kotlin.*
import org.springframework.boot.Banner
import org.springframework.boot.builder.SpringApplicationBuilder
import org.slf4j.LoggerFactory
import java.util.*

data class PlayerRegistration(val identifier: Int, var position: Int? = null)

/** The max number of players that participate in this race. This is limited by the statement. */
const val MAX_PLAYERS = 4
/** The max number of milliseconds the slowest player may run */
const val MAX_MILLISECONDS = 10000
const val SERVER_URL = "http://localhost:8080"
const val STATUS_PATH = "/status"
const val REGISTRATION_PATH = "/registration"
const val FINISH_PATH = "/finish"

val log = LoggerFactory.getLogger(Application::class.java)

data class Race(
    val players: MutableList<PlayerRegistration> = mutableListOf<PlayerRegistration>(),
    var playersArrived: Int = 0
) {
    fun reset() {
        players.clear()
        playersArrived = 0
    }

    fun registerPlayer() : Int? {
        if (players.size == MAX_PLAYERS)
            return null
        players.add(PlayerRegistration(identifier = players.size))
        return players.size - 1
    }

    fun playerFinished(identifier: Int) : Int? {
        if (identifier >= players.size)
            return null
        if (players[identifier].position == null)
            players[identifier].position = ++playersArrived
        return players[identifier].position
    }
}

data class RegistrationResponse(val identifier: Int?)
data class FinishResponse(val position: Int?, val timeElapsed: Long?)

@RestController
class GreetingController {

    val lock = ReentrantLock()
    val race: Race = Race()
    var raceStartTime: Date? = null;
    var condvar: Condition = lock.newCondition()

    /** A dummy path to ensure the app is initialized and listening */
    @RequestMapping(STATUS_PATH)
    fun status(): Boolean {
        log.info("Status request received")
        return true
    }

    @RequestMapping(REGISTRATION_PATH)
    fun register(): RegistrationResponse {
        log.info("Register request received")
        lock.lock()
        val identifier = race.registerPlayer()
        if (identifier == null) {
            lock.unlock()
            return RegistrationResponse(null)
        }

        log.info("Got identifier: %d".format(identifier))

        // This player is registered, just wait until the rest are too.
        if (identifier == MAX_PLAYERS - 1) {
            raceStartTime = Date()
            log.info("Race is starting!")
            condvar.signalAll()
        } else {
            condvar.await()
        }
        lock.unlock()
        return RegistrationResponse(identifier)
    }

    @RequestMapping(FINISH_PATH)
    fun finish(@RequestParam(value = "id") id: Int) : FinishResponse {
        log.info("[%d] Finished".format(id))
        lock.lock() // I miss C++/Rust's RAII
        val position = race.playerFinished(id)
        val date = if (position != null) Date().time - raceStartTime!!.time else null
        if (position == MAX_PLAYERS) {
            log.info("Race is over, resetting...")
            race.reset()
        }
        lock.unlock()
        return FinishResponse(position, date)
    }
}

class PlayerClient {
    fun waitForServer() {
        val statusUrl = URL(SERVER_URL + STATUS_PATH)
        while (true) {
            var connection: HttpURLConnection? = null
            try {
                connection = statusUrl.openConnection() as HttpURLConnection
                connection.requestMethod = "GET"
                if (connection.responseCode == 200)
                    return // All good!
            } catch (e: Exception) {
                // Eat it all
            } finally {
                if (connection != null)
                    connection.disconnect()
                connection = null
            }
        }
    }

    fun registerForRace() : Int {
        val registrationUrl = URL(SERVER_URL + REGISTRATION_PATH)
        val connection = registrationUrl.openConnection() as HttpURLConnection
        connection.requestMethod = "GET"

        val mapper = jacksonObjectMapper()
        val response: RegistrationResponse = mapper.readValue(connection.inputStream)
        connection.disconnect()
        assert(response.identifier != null, { "We raced with someone? How?" })
        return response.identifier!!
    }

    fun race(identifier: Int) : Pair<Int, Long> {
        val millis = Math.floor(Math.random() * MAX_MILLISECONDS).toLong()
        log.info("[%d] is going to sleep for %d milliseconds".format(identifier, millis))
        Thread.sleep(millis)
        val finishUrl = URL(SERVER_URL + FINISH_PATH + "?id=" + identifier)
        val connection = finishUrl.openConnection() as HttpURLConnection
        connection.requestMethod = "GET"

        val mapper = jacksonObjectMapper()
        val response: FinishResponse = mapper.readValue(connection.inputStream)
        connection.disconnect()
        assert(response.position != null, { "We raced? Otherwise this can't happen" })
        return Pair(response.position!!, response.timeElapsed!!)
    }

    fun start() {
        println("Player started")
        waitForServer()
        val identifier = registerForRace()
        log.info("[%d] is registered".format(identifier))
        val (position, timeElapsed) = race(identifier)
        log.info("[%d] Successfully finished at position %d (%d milliseconds)".format(identifier, position, timeElapsed))
    }
}

@SpringBootApplication
open class Application

fun main(args: Array<String>) {
    println("main started")
    for (i in 0 until MAX_PLAYERS)
        Thread { PlayerClient().start() }.start()
    SpringApplicationBuilder()
        .bannerMode(Banner.Mode.OFF)
        .parent(Application::class.java)
        .child(Application::class.java)
        .run(*args)
}
