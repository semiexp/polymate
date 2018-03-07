use super::*;

/// `1` entries and meta nodes in an exact cover problem instance.
///
/// - Node indexing
///   - For starting node, row == 0 and col == 0.
///   - For column headers, row == 0 and col >= 1.
///   - For `1` entries, row >= 1 and col >= 1.
#[derive(Clone, Copy)]
struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,

    row: usize,
    col: usize,
}

pub struct ExactCover {
    nodes: Vec<Node>,
    column_count: Vec<usize>,
    pub n_answers: u64,
}

impl ExactCover {
    pub fn from_dictionary(dic: &Dictionary<u64>) -> ExactCover {
        let mut nodes = vec![];
        let mut col_last = vec![];
        let mut column_count = vec![];

        let n_col = dic.n_target_cells as usize + dic.piece_count.len() + 1;

        for i in 0..n_col {
            nodes.push(Node {
                left: if i == 0 { n_col - 1 } else { i - 1 },
                right: if i == n_col - 1 { 0 } else { i + 1 },
                up: i,
                down: i,
                row: 0,
                col: i,
            });
            col_last.push(i);
            column_count.push(0);
        }

        for c in 0..(dic.n_target_cells as usize) {
            for p in 0..dic.piece_count.len() {
                for &m in &dic.placements[c][p] {
                    let mut cols = vec![p + 1];
                    {
                        let mut m = m;
                        while m != 0 {
                            let idx = m.trailing_zeros();
                            m ^= 1u64 << (idx as u64);
                            cols.push(dic.piece_count.len() + 1 + idx as usize);
                        }
                    }
                    let base = nodes.len();
                    for i in 0..cols.len() {
                        nodes.push(Node {
                            left: if i == 0 { base + cols.len() - 1 } else { base + i - 1 },
                            right: if i == cols.len() - 1 { base } else { base + i + 1 },
                            up: col_last[cols[i]],
                            down: cols[i],
                            row: 0,
                            col: cols[i],
                        });
                        nodes[col_last[cols[i]]].down = nodes.len() - 1;
                        col_last[cols[i]] = nodes.len() - 1;
                        column_count[cols[i]] += 1;
                    }
                }
            }
        }
        for i in 0..n_col {
            nodes[i].up = col_last[i];
        }
        for i in 0..nodes.len() {
            assert_eq!(i, nodes[nodes[i].right].left);
            assert_eq!(i, nodes[nodes[i].down].up);
        }
        ExactCover {
            nodes,
            column_count,
            n_answers: 0u64,
        }
    }
    pub fn search(&mut self) {
        // find the pivot (lightest column)
        let mut cand = (usize::max_value(), 0usize);
        {
            let mut i = self.nodes[0].right;
            while i != 0 {
                cand = std::cmp::min(cand, (self.column_count[i], i));
                i = self.nodes[i].right;
            }
        }

        if cand.0 == 0 { return; }
        let pivot = cand.1;
        if pivot == 0 {
            // TODO: record answer
            self.n_answers += 1;
            return;
        }

        // remove column `pivot` and associated rows
        self.purge_column_full(pivot);

        let mut i = self.nodes[pivot].down;
        while i != pivot {
            // choose the `row` with node `i`
            {
                let mut j = self.nodes[i].right;
                while j != i {
                    let c = self.nodes[j].col;
                    self.purge_column_full(c);
                    j = self.nodes[j].right;
                }
            }

            self.search();

            {
                let mut j = self.nodes[i].left;
                while j != i {
                    let c = self.nodes[j].col;
                    self.restore_column_full(c);
                    j = self.nodes[j].left;
                }
            }
            i = self.nodes[i].down;
        }

        // restore column `pivot` and associated rows
        self.restore_column_full(pivot);
    }

    /// Remove column `c` and all associated rows
    fn purge_column_full(&mut self, c: usize) {
        let mut i = self.nodes[c].down;
        while i != c {
            let mut j = self.nodes[i].right;
            while j != i {
                let u = self.nodes[j].up;
                let d = self.nodes[j].down;
                self.nodes[u].down = d;
                self.nodes[d].up = u;
                self.column_count[self.nodes[j].col] -= 1;

                j = self.nodes[j].right;
            }
            self.column_count[self.nodes[i].col] -= 1;
            i = self.nodes[i].down;
        }

        let l = self.nodes[c].left;
        let r = self.nodes[c].right;
        self.nodes[l].right = r;
        self.nodes[r].left = l;
    }

    /// Undo `purge_column_full`
    fn restore_column_full(&mut self, c: usize) {
        let l = self.nodes[c].left;
        let r = self.nodes[c].right;
        self.nodes[l].right = c;
        self.nodes[r].left = c;

        let mut i = self.nodes[c].up;
        while i != c {
            let mut j = self.nodes[i].right;
            while j != i {
                let u = self.nodes[j].up;
                let d = self.nodes[j].down;
                self.nodes[u].down = j;
                self.nodes[d].up = j;
                self.column_count[self.nodes[j].col] += 1;

                j = self.nodes[j].right;
            }
            self.column_count[self.nodes[i].col] += 1;
            i = self.nodes[i].up;
        }
    }
}
