use super::Solution;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut results: i32 = 0;
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid1[0].len()]; grid1.len()];

        for i in 0..grid1.len() {
            for j in 0..grid1[0].len() {
                if visited[i][j] {
                    continue;
                }
                if grid2[i][j] == 1 && Self::dfs(&grid1, &grid2, &mut visited, i as i32, j as i32) {
                    results += 1;
                }
            }
        }

        results
    }
    pub fn dfs(
        grid1: &[Vec<i32>],
        grid2: &[Vec<i32>],
        visited: &mut [Vec<bool>],
        i: i32,
        j: i32,
    ) -> bool {
        if i < 0
            || j < 0
            || i as usize == grid1.len()
            || j as usize == grid1[0].len()
            || grid2[i as usize][j as usize] == 0
            || visited[i as usize][j as usize]
        {
            return true;
        }

        visited[i as usize][j as usize] = true;
        let mut results = true;
        if grid1[i as usize][j as usize] == 0 {
            results = false;
        }
        results &= Self::dfs(grid1, grid2, visited, i - 1, j);
        results &= Self::dfs(grid1, grid2, visited, i + 1, j);
        results &= Self::dfs(grid1, grid2, visited, i, j + 1);
        results &= Self::dfs(grid1, grid2, visited, i, j - 1);

        results
    }
}
