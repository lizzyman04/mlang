#[derive(Clone, Copy)]
pub struct Symbol {
    pub symbol: char,
    pub label: &'static str,
}

pub fn default_symbols() -> Vec<Symbol> {
    vec![
        Symbol {
            symbol: 'π',
            label: "pi",
        },
        Symbol {
            symbol: '∞',
            label: "infinity",
        },
        Symbol {
            symbol: '%',
            label: "percent",
        },
        Symbol {
            symbol: '√',
            label: "square root",
        },
        Symbol {
            symbol: '∑',
            label: "summation",
        },
        Symbol {
            symbol: '∫',
            label: "integral",
        },
        Symbol {
            symbol: 'α',
            label: "alpha",
        },
        Symbol {
            symbol: 'β',
            label: "beta",
        },
        Symbol {
            symbol: 'γ',
            label: "gamma",
        },
        Symbol {
            symbol: 'θ',
            label: "theta",
        },
        Symbol {
            symbol: 'σ',
            label: "sigma",
        },
        Symbol {
            symbol: 'Δ',
            label: "delta",
        },
        Symbol {
            symbol: 'Ω',
            label: "omega",
        },
        Symbol {
            symbol: 'μ',
            label: "mu",
        },
        Symbol {
            symbol: 'λ',
            label: "lambda",
        },
        Symbol {
            symbol: 'φ',
            label: "phi",
        },
    ]
}
