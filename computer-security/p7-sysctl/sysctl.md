---
title: Práctica 7 - Configuraciones Que Ayudan A La Seguridad
subtitle: Seguridad de Sistemas Informáticos
author:
  - Emilio Cobos Álvarez (70912324N)
  - Juan Carlos Martín García (70882826T)
lang: es
numbersections: true
links-as-notes: true
toc: true
header-includes:
  # Prevent images from floating and getting reordered with the text
  - \usepackage{float}
  - \floatplacement{figure}{H}
  # Make sections start in a new page.
  - \let\Oldsection\section
  - \renewcommand{\section}{\clearpage\Oldsection}
---

# Activando `sshd`

Para instalar el daemon ssh, primero instalamos el servidor de OpenSSH. En
Debian / Ubuntu:

```
# apt install openssh-server
```

Para activar el servicio usamos `systemctl`:

```
# systemctl start sshd
```

Para que el servicio se ejecute al iniciar:

```
# systemctl enable sshd
Created symlink /etc/systemd/system/multi-user.target.wants/sshd.service → /usr/lib/systemd/system/sshd.service.
```

Comprobamos que el servicio está corriendo:

```
# systemctl status sshd
● sshd.service - OpenSSH server daemon
     Loaded: loaded (/usr/lib/systemd/system/sshd.service; enabled; vendor preset: disabled)
     Active: active (running) since Sun 2021-01-03 22:53:34 CET; 2s ago
       Docs: man:sshd(8)
             man:sshd_config(5)
   Main PID: 217821 (sshd)
      Tasks: 1 (limit: 115663)
     Memory: 2.1M
        CPU: 9ms
     CGroup: /system.slice/sshd.service
             └─217821 sshd: /usr/sbin/sshd -D [listener] 0 of 10-100 startups

Jan 03 22:53:34 host systemd[1]: Starting OpenSSH server daemon...
Jan 03 22:53:34 host sshd[217821]: Server listening on 0.0.0.0 port 22.
Jan 03 22:53:34 host sshd[217821]: Server listening on :: port 22.
Jan 03 22:53:34 host systemd[1]: Started OpenSSH server daemon
```

# Utilizando TCP wrappers para controlar el acceso al servidor SSH

Podemos hacer esto de manera muy sencilla utilizando el fichero
`/etc/hosts.allow` para permitir acceso sólo desde la subred:

```
sshd : 192.168.1.*
```

Sin embargo, Fedora, el sistema en el que lo estamos probando esto actualmente,
ha [eliminado el soporte para TCP wrappers hace
años](https://fedoraproject.org/wiki/Changes/Deprecate_TCP_wrappers)

Sin embargo, podemos migrar a `tcpd` como se describe en el link anterior,
o usar systemd editando el fichero de servicio así para permitir acceso sólo
desde la red local:

```diff
diff --git a/sshd.service.bak b/sshd.service
index e8afb86..55cd34a 100644
--- a/sshd.service.bak
+++ b/sshd.service
@@ -12,6 +12,8 @@ ExecReload=/bin/kill -HUP $MAINPID
 KillMode=process
 Restart=on-failure
 RestartSec=42s
+IPAddressAllow=192.168.0.0/24
+IPAddressDeny=any
 
 [Install]
 WantedBy=multi-user.target
```

Tras esto, reiniciamos el servicio con:

```
# systemctl daemon-reload
# systemctl restart sshd
```

Cambiando el filtro para comprobar que funciona a una IP inexistente podemos
comprobar como desde otro ordenador bloquea correctamente el acceso (el cliente
ssh se desconecta por un timeout).

# Parámetros `sysctl` para la seguridad en redes.

Podemos ver múltiples variables de configuración del kernel relativo a la red
con:

```
# sysctl -a | grep net.ipv
net.ipv4.cipso_cache_bucket_size = 10
net.ipv4.cipso_cache_enable = 1
net.ipv4.cipso_rbm_optfmt = 0
net.ipv4.cipso_rbm_strictvalid = 1
net.ipv4.conf.all.accept_local
...
```

Algunas de ellas ayudan a mitigar ciertos tipos de ataque, por ejemplo:

```
# Valida paquetes usando reversed path, evitando IP spoofing.
# https://tools.ietf.org/html/rfc1812
net.ipv4.conf.all.rp_filter = 1

# Previene el 'syn flood attack', see:
#
# https://en.wikipedia.org/wiki/SYN_flood
# https://en.wikipedia.org/wiki/SYN_cookies
net.ipv4.tcp_syncookies = 1
```

Tras editar sysctl.conf, podemos ejecutar `sysctl -p` para recargarlo en el
kernel actual sin reiniciar.
