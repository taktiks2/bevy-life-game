//! ライフゲームの有名パターン定義
//!
//! メニュー画面からワールドに配置できるパターンのデータを提供する。

/// ライフゲームの有名パターン
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum LifePattern {
    /// パターンなし（デフォルト）
    #[default]
    None,
    /// グライダー（移動体）
    Glider,
    /// 軽量宇宙船（移動体）
    Lwss,
    /// パルサー（振動子・周期3）
    Pulsar,
    /// ゴスパーグライダー銃
    GosperGliderGun,
    /// Rペントミノ（メトセラ）
    RPentomino,
    /// エイコーン（メトセラ）
    Acorn,
}

impl LifePattern {
    /// パターンのセル座標（原点中心）を返す
    pub fn cells(&self) -> &'static [(i32, i32)] {
        match self {
            Self::None => &[],
            Self::Glider => &[(0, -1), (1, 0), (-1, 1), (0, 1), (1, 1)],
            Self::Lwss => &[
                (-1, -1),
                (2, -1),
                (-2, 0),
                (-2, 1),
                (2, 1),
                (-2, 2),
                (-1, 2),
                (0, 2),
                (1, 2),
            ],
            Self::Pulsar => {
                // 周期3の振動子 — 4象限対称
                &[
                    // 上部（y=-6行）
                    (-4, -6),
                    (-3, -6),
                    (-2, -6),
                    (2, -6),
                    (3, -6),
                    (4, -6),
                    // y=-4行
                    (-6, -4),
                    (-1, -4),
                    (1, -4),
                    (6, -4),
                    // y=-3行
                    (-6, -3),
                    (-1, -3),
                    (1, -3),
                    (6, -3),
                    // y=-2行
                    (-6, -2),
                    (-1, -2),
                    (1, -2),
                    (6, -2),
                    // y=-1行
                    (-4, -1),
                    (-3, -1),
                    (-2, -1),
                    (2, -1),
                    (3, -1),
                    (4, -1),
                    // y=1行
                    (-4, 1),
                    (-3, 1),
                    (-2, 1),
                    (2, 1),
                    (3, 1),
                    (4, 1),
                    // y=2行
                    (-6, 2),
                    (-1, 2),
                    (1, 2),
                    (6, 2),
                    // y=3行
                    (-6, 3),
                    (-1, 3),
                    (1, 3),
                    (6, 3),
                    // y=4行
                    (-6, 4),
                    (-1, 4),
                    (1, 4),
                    (6, 4),
                    // y=6行
                    (-4, 6),
                    (-3, 6),
                    (-2, 6),
                    (2, 6),
                    (3, 6),
                    (4, 6),
                ]
            }
            Self::GosperGliderGun => &[
                // 左ブロック
                (0, 4),
                (0, 5),
                (1, 4),
                (1, 5),
                // 左部品
                (10, 4),
                (10, 5),
                (10, 6),
                (11, 3),
                (11, 7),
                (12, 2),
                (12, 8),
                (13, 2),
                (13, 8),
                (14, 5),
                (15, 3),
                (15, 7),
                (16, 4),
                (16, 5),
                (16, 6),
                (17, 5),
                // 右部品
                (20, 2),
                (20, 3),
                (20, 4),
                (21, 2),
                (21, 3),
                (21, 4),
                (22, 1),
                (22, 5),
                (24, 0),
                (24, 1),
                (24, 5),
                (24, 6),
                // 右ブロック
                (34, 2),
                (34, 3),
                (35, 2),
                (35, 3),
            ],
            Self::RPentomino => &[(0, -1), (1, -1), (-1, 0), (0, 0), (0, 1)],
            Self::Acorn => &[(-3, 1), (-2, -1), (-2, 1), (0, 0), (1, 1), (2, 1), (3, 1)],
        }
    }

    /// UI表示用ラベルを返す
    pub fn label(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Glider => "Glider",
            Self::Lwss => "LWSS",
            Self::Pulsar => "Pulsar",
            Self::GosperGliderGun => "Glider Gun",
            Self::RPentomino => "R-pentomino",
            Self::Acorn => "Acorn",
        }
    }

    /// 全パターン一覧（None除外）を返す
    pub fn all() -> &'static [LifePattern] {
        &[
            Self::Glider,
            Self::Lwss,
            Self::Pulsar,
            Self::GosperGliderGun,
            Self::RPentomino,
            Self::Acorn,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_returns_six_patterns() {
        let patterns = LifePattern::all();
        assert_eq!(patterns.len(), 6);
        assert!(!patterns.contains(&LifePattern::None));
    }

    #[test]
    fn glider_has_5_cells() {
        assert_eq!(LifePattern::Glider.cells().len(), 5);
    }

    #[test]
    fn lwss_has_9_cells() {
        assert_eq!(LifePattern::Lwss.cells().len(), 9);
    }

    #[test]
    fn pulsar_has_48_cells() {
        assert_eq!(LifePattern::Pulsar.cells().len(), 48);
    }

    #[test]
    fn gosper_glider_gun_has_36_cells() {
        assert_eq!(LifePattern::GosperGliderGun.cells().len(), 36);
    }

    #[test]
    fn r_pentomino_has_5_cells() {
        assert_eq!(LifePattern::RPentomino.cells().len(), 5);
    }

    #[test]
    fn acorn_has_7_cells() {
        assert_eq!(LifePattern::Acorn.cells().len(), 7);
    }

    #[test]
    fn labels_are_not_empty() {
        for pattern in LifePattern::all() {
            assert!(!pattern.label().is_empty(), "{:?} has empty label", pattern);
        }
    }

    #[test]
    fn none_cells_is_empty() {
        assert_eq!(LifePattern::None.cells().len(), 0);
    }

    #[test]
    fn default_is_none() {
        assert_eq!(LifePattern::default(), LifePattern::None);
    }
}
