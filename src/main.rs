use gol::*;
use std::io::{self, Write};

fn main() {
    let mut buf = String::new();
    let width;
    let height;
    
    print!("board width: ");
    io::stdout().flush().unwrap();
    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        println!();
        if let Ok(parsed) = buf.trim().parse() {
            width = parsed;
            break;
        } else {
            print!("try again: ");
            io::stdout().flush().unwrap();
        }
    }
    
    print!("board height: ");
    io::stdout().flush().unwrap();
    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        println!();
        if let Ok(parsed) = buf.trim().parse() {
            height = parsed;
            break;
        } else {
            print!("try again: ");
            io::stdout().flush().unwrap();
        }
    }

    let board_size = (width, height).into();
    let mut board = board::Board::new(board_size, true);

    println!("type in the beginning state");
    println!("# for a live cell and space for a dead cell");
    println!("<EOT> to skip the rest of rows");
    
    'readlines: for y in 0..height {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        for (x, c) in line.bytes().enumerate() {
            if x >= width.into() { break }
            if c == b'#' {
                board.set((x as u16, y).into(), true);
            } else if c == 4 {
                break 'readlines
            }
        }
    }
    
    let mut renderer = render::Renderer::new(10, board_size);
    
    loop {
        board.cont();
        renderer.render(&board);
        renderer.handle_events();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
