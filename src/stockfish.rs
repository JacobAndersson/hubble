use std::io::{Write, Read};
use std::process::{Command, Stdio, Child, ChildStdout, ChildStdin};

#[derive(Debug)]
pub struct Scores {
    pub classic: Option<f32>,
    pub nnue: Option<f32>,
    pub combined: Option<f32>
}

pub struct Stockfish {
    engine: Child,
    stdout: ChildStdout,
    stdin: ChildStdin
}

impl Stockfish {
    pub fn new() -> Result<Self, ()> {
        let mut engine = Command::new("./stockfish").stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().unwrap();
        let stdout = engine.stdout.take().unwrap();
        let stdin = engine.stdin.take().unwrap();

        Ok(Self { 
            engine,
            stdout, 
            stdin
        })
    }
    
    pub fn read_output(&mut self, min_length: Option<usize>) -> String {
        loop {
            let mut buffer = [0; 3920];
            self.stdout.read(&mut buffer[..]);
            let output_string = String::from_utf8(buffer.to_vec()).unwrap();
            let count = buffer.iter().filter(|&n| n != &0).count();

            if count >= min_length.unwrap_or(0) {
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

    fn parse_eval(&self, output: String) -> Scores {
        let lines = output
            .trim_matches(char::from(0))
            .split("\n")
            .filter(|x| x.len() > 1)
            .collect::<Vec<&str>>(); 

        let length = lines.len();
        let evals = &lines[length - 3..];

        let mut scores: Vec<Option<f32>> = Vec::new();
        for eval in evals {
            scores.push(self.parse_score(eval));
        }


        Scores {
            classic: scores[0],
            nnue: scores[1],
            combined: scores[2]
        }
    }

    pub fn eval_fen(&mut self, fen: &str) -> Scores {
        let input_string = format!("position fen {}\n eval \n", fen);
        self.stdin.write(input_string.as_bytes());
        let output = self.read_output(Some(3000));
        self.parse_eval(output)
    }

    pub fn flush(&mut self) {
        self.read_output(None);
    }
}
