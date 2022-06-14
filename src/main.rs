use raylib::prelude::*;


const SQUARE_SIZE: i32 = 200;
const SCREEN_WIDTH: i32 = 600;
const SCREEN_HEIGHT: i32 = 800;

fn main() {
    let mut turn: i32 = 1;
    let mut field: [[i32; 3]; 3] = [[0; 3]; 3];
    let  (mut p1_points, mut p2_points) = (0, 0);

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raylib Tic Tac Toe, written in Rust")
        .build();

    field = reset_game(field);

    rl.set_target_fps(60);

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);

        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let c_field: i32 = get_field(&d);
        	println!("Pressed Field: {}", c_field);

        	if c_field != 0 {
        		if c_field == 1 && field[0][0] == 0 {field[0][0] = turn;}
        		else if c_field == 2 && field[1][0] == 0 {field[1][0] = turn;}
        		else if c_field == 3 && field[2][0] == 0 {field[2][0] = turn;}
        		else if c_field == 4 && field[0][1] == 0 {field[0][1] = turn;}
        		else if c_field == 5 && field[1][1] == 0 {field[1][1] = turn;}
        		else if c_field == 6 && field[2][1] == 0 {field[2][1] = turn;}
        		else if c_field == 7 && field[0][2] == 0 {field[0][2] = turn;}
        		else if c_field == 8 && field[1][2] == 0 {field[1][2] = turn;}
        		else if c_field == 9 && field[2][2] == 0 {field[2][2] = turn;}

        		if turn == 1 {turn = 2;} else {turn = 1;}

                /*
                for i in 0..3 {
    				for j in 0..3 {
    					print!("{}", field[i][j]);
    				}
    			}

                print!("\n");
                */

    			if check_winner(field) != 0 {
    				if check_winner(field) == 1 {
    					println!("Player 1 wins!");
    					p1_points += 1;
    				} else if check_winner(field) == 2 {
    					println!("Player 2 wins!");
    					p2_points += 1;
        			} else if check_winner(field) == 3 {
                        println!("Draw!");
                    }
        			field = reset_game(field);
                    turn = 1;
        		}
        	}
        }



        d.clear_background(Color::RAYWHITE);

        for i in  0..(SCREEN_WIDTH/SQUARE_SIZE + 1) {
            d.draw_line(SQUARE_SIZE*i, SQUARE_SIZE, SQUARE_SIZE*i, SCREEN_HEIGHT, Color::DARKGRAY);
        }

        for i in  0..(SCREEN_HEIGHT/SQUARE_SIZE + 1) {
            d.draw_line(0, 200 * (i + 1), SCREEN_WIDTH, SQUARE_SIZE * (i + 1), Color::DARKGRAY);
        }

        d.draw_text(format!("Player {}'s turn", turn).as_str(), 5, 5, 40, Color::DARKGRAY);
        d.draw_text(format!("Player 1: {}", p1_points).as_str(), 5, 50, 40, Color::DARKGRAY);
        d.draw_text(format!("Player 2: {}", p2_points).as_str(), 5, 95, 40, Color::DARKGRAY);

        for i in 0..3 {
          	for j in 0..3 {
           		if field[i][j] == 1 {d.draw_circle(i as i32 * SQUARE_SIZE + 100, j as i32 * SQUARE_SIZE + 300, 50.0, Color::RED);}
                else if field[i][j] == 2 {d.draw_circle(i as i32 * SQUARE_SIZE + 100, j as i32 * SQUARE_SIZE + 300, 50.0, Color::BLUE);}
    	    }
        }
    }
}

fn get_field(d: &RaylibDrawHandle)-> i32 {
	if d.get_mouse_x() >= 0 && d.get_mouse_x() <= SQUARE_SIZE - 1 && d.get_mouse_y() >= SQUARE_SIZE + 1 && d.get_mouse_y() <= 2 * SQUARE_SIZE - 1 {
		return 1;
	} else if d.get_mouse_x() >= SQUARE_SIZE && d.get_mouse_x() <= 2 * SQUARE_SIZE - 1 && d.get_mouse_y() >= SQUARE_SIZE + 1 && d.get_mouse_y() <= 2 * SQUARE_SIZE - 1 {
		return 2;
	} else if d.get_mouse_x() >= 2 * SQUARE_SIZE && d.get_mouse_x() <= 3 * SQUARE_SIZE - 1 && d.get_mouse_y() >= SQUARE_SIZE + 1 && d.get_mouse_y() <= 2 * SQUARE_SIZE - 1 {
		return 3;
	} else if d.get_mouse_x() >= 0 && d.get_mouse_x() <= SQUARE_SIZE - 1 && d.get_mouse_y() >= 2 * SQUARE_SIZE + 1 && d.get_mouse_y() <= 3 * SQUARE_SIZE - 1 {
		return 4;
	} else if d.get_mouse_x() >= SQUARE_SIZE && d.get_mouse_x() <= 2 * SQUARE_SIZE - 1 && d.get_mouse_y() >= 2 * SQUARE_SIZE + 1 && d.get_mouse_y() <= 3 * SQUARE_SIZE - 1 {
		return 5;
	} else if d.get_mouse_x() >= 2 * SQUARE_SIZE && d.get_mouse_x() <= 3 * SQUARE_SIZE - 1 && d.get_mouse_y() >= 2 * SQUARE_SIZE + 1 && d.get_mouse_y() <= 3 * SQUARE_SIZE - 1 {
		return 6;
	} else if d.get_mouse_x() >= 0 && d.get_mouse_x() <= SQUARE_SIZE - 1 && d.get_mouse_y() >= 3 * SQUARE_SIZE + 1 && d.get_mouse_y() <= 4 * SQUARE_SIZE - 1 {
		return 7;
	} else if d.get_mouse_x() >= SQUARE_SIZE && d.get_mouse_x() <= 2 * SQUARE_SIZE - 1 && d.get_mouse_y() >= 3 * SQUARE_SIZE + 1 && d.get_mouse_y() <= 4 * SQUARE_SIZE - 1 {
		return 8;
	} else if d.get_mouse_x() >= 2 * SQUARE_SIZE && d.get_mouse_x() <= 3 * SQUARE_SIZE - 1 && d.get_mouse_y() >= 3 * SQUARE_SIZE + 1 && d.get_mouse_y() <= 4 * SQUARE_SIZE - 1 {
		return 9;
	} else {
		return 0;
	}
}

fn check_winner(field: [[i32; 3]; 3])-> i32 {
	//Check horizontal
    for i in 0..3 {
		if field[0][i] != 0 && field[0][i] == field[1][i] && field[0][i] == field[2][i] {
			return field[0][i];
		}
    }

    //Check vertical
    for i in 0..3 {
    	if field[i][0] != 0 && field[i][0] == field[i][1] && field[i][0] == field[i][2] {
    		return field[i][0];
    	}
    }

    //Check diagonal
    if field[0][0] != 0 && field[0][0] == field[1][1] && field[0][0] == field[2][2] {
    	return field[1][1];
    } else if field[2][0] != 0 && field[2][0] == field[1][1] && field[2][0] == field[0][2] {
    	return field[1][1];
    }

    //Check for Draw
	let mut d: bool = true;
	for i in 0..3 {
    		for j in 0..3 {
    			if field[i][j] == 0 {d = false;}
    		}
    }
    if d {return 3;}

    return 0;
}

fn reset_game(mut field: [[i32; 3]; 3])-> [[i32; 3]; 3] {
	for i in 0..3 {
    		for j in 0..3 {
    			field[i][j] = 0;
    		}
    }
    return field;
}
