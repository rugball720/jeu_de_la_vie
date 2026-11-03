use std::thread;
use std::time::Duration;

fn create_matrix(n: usize) -> Vec<Vec<char>> {
	vec![vec!['.'; n]; n]
}

fn print_matrix(matrix: &Vec<Vec<char>>) -> () {
    for line in matrix {
        for element in line {
            print!("{} ",element);
        }
        println!();
    }
    println!()
}

fn change_case(i:usize,j:usize,tab: &Vec<Vec<char>>)-> char {
    let mut result = '.';
    let case:char = tab[i][j];
    if i==0 || j==0 || i ==tab.len() || j== tab.len() {
        return '.'
    }
    else{
            let mut comptage=0;
            if tab[i-1][j-1]=='#'{
                comptage+=1;
            }
            if tab[i-1][j]=='#'{
                comptage+=1;
            }
            if tab[i-1][j+1]=='#'{
                comptage+=1;
            }
            if tab[i][j-1]=='#'{
                comptage+=1;
            }
            if tab[i][j+1]=='#'{
                comptage+=1;
            }
            if tab[i+1][j-1]=='#'{
                comptage+=1;
            }
            if tab[i+1][j]=='#'{
                comptage+=1;
            }
            if tab[i+1][j+1]=='#'{
                comptage+=1;
            }
            if comptage==3 && case=='.'{
                result='#';
            }
            if case=='#'{
                if comptage==3 || comptage==2{
                    result='#';
                }
            }
        }
    return result
}

fn update(old: &Vec<Vec<char>>, new: &mut Vec<Vec<char>>) -> () {
    let n = old.len();
    for i in 0..n-1 {
        for j in 0..n-1 {
            new[i][j] = change_case(i,j,old);
        }
    }
}

fn play(matrix: &Vec<Vec<char>>) -> () {
    let n = matrix.len();
    let mut old = matrix.clone();
    let mut new = create_matrix(n);
    while old != new {
        print_matrix(&old);
        thread::sleep(Duration::from_secs(1));
        update(&old, &mut new);
        old = new;
        new = create_matrix(n);
    }
    print_matrix(&old);
}

fn main() {
    let mut matrix = create_matrix(5);
    matrix[1][2] = '#';
    matrix[2][2] = '#';
    matrix[3][2] = '#';
    
    play(&matrix);
}