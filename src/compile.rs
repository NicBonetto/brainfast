#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
  Add,        // +
  Sub,        // -
  Right,      // >
  Left,       // <
  Read,       // ,
  Write,      // .
  BeginLoop,  // [
  EndLoop,    // ]
}

pub fn tokenize(input: &str) -> Vec<Token> {
  let mut tokens = Vec::<Token>::new();
  let mut chars = input.chars();

  while let Some(c) = chars.next() {
    match c {
      '+' => tokens.push(Token::Add),
      '-' => tokens.push(Token::Sub),
      '>' => tokens.push(Token::Right),
      '<' => tokens.push(Token::Left),
      ',' => tokens.push(Token::Read),
      '.' => tokens.push(Token::Write),
      '[' => tokens.push(Token::BeginLoop),
      ']' => tokens.push(Token::EndLoop),
      _ => panic!("Could not finish compiling. Unknown symbol: {}", c.to_string()),
    }
  }

  tokens
}

fn indent(count: u32) -> String {
  let mut indentation = String::new();

  for _ in 0..count {
    indentation.push_str("  ");  // 2-space indent
  }
  indentation
}

pub fn generate(tokens: &[Token]) -> String {
  let mut output = String::from("int main() {\n");
  let mut indent_count = 1u32;

  for &token in tokens {
    match token {
      Token::Add => {
        // Increment value at selected cell
        output.push_str(&indent(indent_count));
        output.push_str("++*ptr;\n");
      }
      Token::Sub => {
        // Decrement value at selected cell
        output.push_str(&indent(indent_count));
        output.push_str("--*ptr;\n");
      }
      Token::Right => {
        // Change selected cell one to the right
        output.push_str(&indent(indent_count));
        output.push_str("++ptr;\n");
      }
      Token::Left => {
        // Change selected cell one to the left
        output.push_str(&indent(indent_count));
        output.push_str("--ptr;\n");
      }
      Token::Read => {
        // Read a single character into the selected cell
        output.push_str(&indent(indent_count));
        output.push_str("*ptr=getchar();\n");
      }
      Token::Write => {
        // Print character at selected cell
        output.push_str(&indent(indent_count));
        output.push_str("putchar(*ptr);\n");
      }
      Token::BeginLoop => {
        // Begin a loop at the current cell
        output.push_str(&indent(indent_count));
        output.push_str("while (*ptr) {\n");
        indent_count += 1;
      }
      Token::EndLoop => {
        // End a loop
        indent_count -= 1;
        output.push_str(&indent(indent_count));
        output.push_str("}\n");
      }
    }
  }

  output.push_str("}\n");
  output
}

// Unit tests
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_tokenize() {
    assert_eq!(tokenize(&String::from("[+-><.,]")), vec![Token::BeginLoop, Token::Add, Token::Sub, Token::Right, Token::Left, Token::Write, Token::Read, Token::EndLoop]);
  }

  #[test]
  #[should_panic]
  fn test_bad_tokenize() {
    tokenize(&String::from("+<-*"));
  }

  #[test]
  fn test_indent() {
    assert_eq!(indent(2), String::from("    "));
  }

  #[test]
  fn test_generate() {
    assert_eq!(generate(&vec![Token::BeginLoop, Token::Add, Token::Sub, Token::Right, Token::EndLoop]), String::from("int main() {\n  while (*ptr) {\n    ++*ptr;\n    --*ptr;\n    ++ptr;\n  }\n}\n"));
  }
}