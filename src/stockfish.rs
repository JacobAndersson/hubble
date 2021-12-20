use std::io::{Write, Read, BufRead};
use std::process::{Command, Stdio, Child, ChildStdin};

#[derive(Debug)]
pub struct Scores {
    pub classic: Option<f32>,
    pub nnue: Option<f32>,
    pub combined: Option<f32>
}

pub struct Stockfish {
    engine: Child,
    stdout: Box<dyn BufRead>,
    stdin: ChildStdin
}

impl Stockfish {
    pub fn new() -> Result<Self, ()> {
        let mut engine = Command::new("./stockfish").stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().unwrap();

        let temp = engine.stdout.take().unwrap();
        let stdin = engine.stdin.take().unwrap();
        let stdout = std::io::BufReader::new(temp);

        Ok(Self {
            engine,
            stdout: Box::new(stdout),
            stdin
        })
    }
    
    pub fn read_output(&mut self) -> String {
        loop {
            let mut buffer = [0; 3920];
            self.stdout.read(&mut buffer);
            let output_string = String::from_utf8(buffer.to_vec()).unwrap();
            if output_string.contains("info depth 20") && output_string.contains("score cp") {
                return output_string;
            }
        }
    }

    fn parse_score(&self, eval: &str) -> Option<f32> {
       let words =  eval.split(" ").filter(|x| x.len() > 0).collect::<Vec<&str>>();
       let raw_score = &words[2] 
           .replace("+", "")
           .replace("-", "")
           .parse::<f32>();

        match raw_score {
            Ok(score) => Some(*score),
            Err(_) => None
        }
    }

    fn parse_eval(&self, output: String) -> Option<f32> {
        let words = output.split(" ").collect::<Vec<&str>>();
        match words.iter().position(|x| x == &"cp") {
            Some(idx) => {
                match words[idx + 1].parse::<f32>() {
                    Ok(s) => Some(s),
                    Err(_) => None
                }
            },
            None => None
        }
    }

    fn is_ready(&mut self) {
        writeln!(self.stdin, "isready");
        let out = self.read_output();
    }

    pub fn eval_fen(&mut self, fen: &str) -> Option<f32> {
        writeln!(self.stdin, "position fen {}\ngo depth {}\n", fen, 20).unwrap();
        let output = self.read_output();
        let score = self.parse_eval(output);
        score
    }

    pub fn flush(&mut self) {
        self.read_output();
    }
}
