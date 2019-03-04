fn tree_height(root: &i32, tree: &Vec<Vec<i32>>) -> i32 {
    1 + tree[*root as usize].iter().fold(0, |acc, v| {
        let proposed = tree_height(v, tree);
        if proposed > acc {
            proposed
        } else {
            acc
        }
    })
}

fn main() {
    let mut buff = String::with_capacity(1);
    ::std::io::stdin().read_line(&mut buff).unwrap();

    let input_size: usize = buff.trim().parse().unwrap();

    buff.clear();
    buff.reserve_exact(input_size * 2);

    ::std::io::stdin().read_line(&mut buff).unwrap();

    let mut root: i32 = -1;
    let mut children: Vec<Vec<i32>> = std::iter::repeat(vec![]).take(input_size).collect();

    let mut acc: i32 = 0;
    for c in buff.split_whitespace() {
        let n: i32 = c.parse().unwrap();
        if n == -1 {
            root = acc;
        } else {
            children
                .get_mut(n as usize)
                .map(|child_vec| child_vec.push(acc))
                .unwrap_or_else(|| {
                    children[n as usize] = vec![acc];
                });
        }
        acc = acc + 1;
    }

    println!("{}", tree_height(&root, &children));
}
