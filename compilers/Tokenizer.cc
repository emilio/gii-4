#include "Tokenizer.h"
#include <string>

static bool isWhitespace(char which) {
  // Well, unicode people won't love it, but for a simple school assignment,
  // this can be enough.
  return which == '\n' || which == '\t' || which == ' ';
}

static bool isNumeric(char which) {
  return which >= '0' && which <= '9';
}

static bool isAlphabetic(char which) {
  return (which >= 'a' && which <= 'z') || (which >= 'A' && which <= 'Z');
}

static bool isIdentifierStart(char which) {
  return which == '_' || isAlphabetic(which);
}

static bool isIdentPart(char which) {
  return isIdentifierStart(which) || isNumeric(which);
}

static bool isOperator(char which) {
  return which == '+' || which == '*' || which == '-' || which == '/';
}

static bool isTokenSeparator(char which) {
  return isWhitespace(which) || isOperator(which) || which == '(' ||
         which == ')' || !which;
}

char Tokenizer::peekChar() {
  if (m_savedChars.empty())
    m_savedChars.push_back(m_reader.next());
  return m_savedChars[0];
}

char Tokenizer::nextChar() {
  char which = peekChar();
  m_savedChars.erase(m_savedChars.begin());
  if (which)
    m_location.column += 1;
  if (which == '\n') {
    m_location.column = 0;
    m_location.line += 1;
  }
  return which;
}

Token Tokenizer::nextToken() {
again:
  Span location = m_location;
  char next = nextChar();
  if (!next)
    return Token::createOfType(TokenType::Eof, location);

  if (isWhitespace(next))
    goto again;

  if (isOperator(next))
    return Token::createOp(next, location);

  if (next == ')')
    return Token::createOfType(TokenType::RightParen, location);

  if (next == '(')
    return Token::createOfType(TokenType::LeftParen, location);

  if (isNumeric(next)) {
    std::string number;
    number.push_back(next);
    // TODO(emilio): We could look for hexadecimal bases and similar here, but
    // meh.
    while (isNumeric(peekChar()))
      number.push_back(nextChar());
    if (!isTokenSeparator(peekChar())) {
      // TODO(emilio): Return an error, here and below.
      fprintf(stderr, "Invalid token after number %c\n", peekChar());
      abort();
    }
    return Token::createNumber(std::stoull(number), location);
  }

  if (isIdentifierStart(next)) {
    std::string ident;
    ident.push_back(next);

    while (isIdentPart(peekChar()))
      ident.push_back(nextChar());

    if (!isTokenSeparator(peekChar())) {
      fprintf(stderr, "Invalid token after ident %c\n", peekChar());
      abort();
    }

    return Token::createIdent(ident.c_str(), location);
  }

  fprintf(stderr, "Unknown token: %c\n", next);
  abort();

  return Token::createOfType(TokenType::Eof, location);
}
