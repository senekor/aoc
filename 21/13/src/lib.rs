fn paper_dims(dots: &[(usize, usize)]) -> (usize, usize) {
    let (max_x, max_y) = dots.iter().fold((0, 0), |(max_x, max_y), (x, y)| match () {
        _ if *x > max_x && *y > max_y => (*x, *y),
        _ if *x > max_x => (*x, max_y),
        _ if *y > max_y => (max_x, *y),
        _ => (max_x, max_y),
    });
    (max_x + 1, max_y + 1)
}

fn fold(mut paper: Vec<Vec<char>>, axis: String, n: usize) -> Vec<Vec<char>> {
    if axis == *"y" {
        for row in &mut paper {
            for j in 0..n {
                if row[j] == '#' {
                    continue;
                }
                let mirror = n + (n - j);
                row[j] = row[mirror];
            }
            *row = row.clone().into_iter().take(n).collect::<Vec<_>>();
        }
    } else {
        for i in 0..n {
            let mirror = n + (n - i);
            for j in 0..paper[i].len() {
                if paper[i][j] == '#' {
                    continue;
                }
                paper[i][j] = paper[mirror][j];
            }
        }
        paper = paper.into_iter().take(n).collect::<Vec<_>>();
    };
    paper
}

type Dots = Vec<(usize, usize)>;
type Folds = Vec<(String, usize)>;

fn get_dots_folds(input: &str) -> (Dots, Folds) {
    let dots = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split(',');
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let folds = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            let last = line.split(' ').nth(2).unwrap();
            let mut iter = last.split('=');
            (
                iter.next().unwrap().to_string(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    (dots, folds)
}

pub fn part1(input: &str) -> usize {
    let (dots, folds) = get_dots_folds(input);

    let (max_x, max_y) = paper_dims(&dots);

    let mut paper = vec![vec!['.'; max_y]; max_x];
    for (i, j) in dots {
        paper[i][j] = '#';
    }

    let first_fold = &folds[0];

    paper = fold(paper, first_fold.0.to_owned(), first_fold.1);

    let num_dots = paper
        .iter()
        .flat_map(|line| line.iter().filter(|c| **c == '#'))
        .count();

    num_dots
}

pub fn part2(input: &str) -> String {
    let (dots, folds) = get_dots_folds(input);

    let (max_x, max_y) = paper_dims(&dots);

    let mut paper = vec![vec!['.'; max_y]; max_x];
    for (i, j) in dots {
        paper[i][j] = '#';
    }

    for foldd in folds {
        paper = fold(paper, foldd.0, foldd.1)
    }

    let mut res = String::new();
    for j in 0..paper[0].len() {
        for row in &paper {
            res.push(row[j]);
        }
        res.push('\n');
    }
    res
}
