#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Pins(u16);

impl Pins {
    /// The sole constructor: >10 pins is an illegal single roll.
    fn new(n: u16) -> Result<Self, Error> {
        if n > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else {
            Ok(Self(n))
        }
    }
    const fn value(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Frame {
    /// First ball cleared the rack (no second ball exists).
    Strike,
    /// Both balls cleared the rack; `first` is necessarily 1..=9 (the second
    /// ball is derivable as `10 - first`, so it is not stored).
    Spare(Pins),
    /// Two balls that together knocked down fewer than 10.
    Open(Pins, Pins),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TenthFrame {
    /// Strike + two bonus balls (each 0..=10).
    Strike { bonus1: Pins, bonus2: Pins },
    /// Spare (first 1..=9) + one bonus ball.
    Spare { first: Pins, bonus: Pins },
    /// Open frame, no bonus.
    Open { first: Pins, second: Pins },
}

impl TenthFrame {
    /// Total pins for the 10th frame only (its bonus balls are credited to
    /// itself, since no later frame exists).
    const fn total(&self) -> u16 {
        match self {
            TenthFrame::Strike { bonus1, bonus2 } => 10 + bonus1.value() + bonus2.value(),
            TenthFrame::Spare { first: _, bonus } => 10 + bonus.value(),
            TenthFrame::Open { first, second } => first.value() + second.value(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum PartialFrame {
    #[default]
    Empty,
    /// One ball thrown that was *not* a strike (1..=9); awaiting the second.
    Half(Pins),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum PartialTenthFrame {
    #[default]
    Empty,
    /// One ball thrown, not a strike (1..=9); rack remaining = 10 - a.
    Open1(Pins),
    /// Strike thrown; the rack is reset to 10 for the first bonus ball.
    StrikeBonus1,
    /// Strike + first bonus ball `b` thrown; awaiting the last bonus.
    /// Rack remaining = 10 if `b` was a strike, else `10 - b`.
    StrikeBonus2(Pins),
    /// Spare made (`first` + second == 10, first != 10); rack reset for bonus.
    SpareBonus(Pins),
}

#[derive(Debug, Default)]
pub struct BowlingGame {
    /// Completed regular frames (0..=9).
    frames: Vec<Frame>,
    /// The regular frame currently accepting balls.
    pending: PartialFrame,
    /// The 10th frame under construction (active once `frames.len() == 9`).
    tenth_pending: PartialTenthFrame,
    /// The completed 10th frame, once all its balls have been rolled.
    tenth: Option<TenthFrame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let p = Pins::new(pins)?;
        if self.tenth.is_some() {
            return Err(Error::GameComplete);
        }
        if self.frames.len() < 9 {
            self.roll_regular(p)
        } else {
            self.roll_tenth(p)
        }
    }

    pub fn score(&self) -> Option<u16> {
        // Need all nine regular frames *and* a complete 10th frame.
        if self.frames.len() < 9 || self.tenth.is_none() {
            return None;
        }
        let tenth = self.tenth.as_ref().expect("checked above");

        // Flatten every ball into one sequence. The 10th frame always
        // contributes >= 2 balls, so a 9th-frame strike is guaranteed its two
        // bonus balls from this sequence.
        let mut rolls: Vec<u16> = Vec::with_capacity(21);
        for fr in &self.frames {
            match fr {
                Frame::Strike => rolls.push(10),
                Frame::Spare(first) => {
                    rolls.push(first.value());
                    rolls.push(10 - first.value());
                }
                Frame::Open(a, b) => {
                    rolls.push(a.value());
                    rolls.push(b.value());
                }
            }
        }
        match tenth {
            TenthFrame::Strike { bonus1, bonus2 } => {
                rolls.push(10);
                rolls.push(bonus1.value());
                rolls.push(bonus2.value());
            }
            TenthFrame::Spare { first, bonus } => {
                rolls.push(first.value());
                rolls.push(10 - first.value());
                rolls.push(bonus.value());
            }
            TenthFrame::Open { first, second } => {
                rolls.push(first.value());
                rolls.push(second.value());
            }
        }

        // Walk frames 1..=9, crediting strike/spare bonuses from the upcoming
        // balls, then add the self-contained 10th-frame total.
        let mut total: u16 = 0;
        let mut idx: usize = 0;
        for fr in &self.frames {
            match fr {
                Frame::Strike => {
                    total += 10 + rolls[idx + 1] + rolls[idx + 2];
                    idx += 1;
                }
                Frame::Spare(_) => {
                    total += 10 + rolls[idx + 2];
                    idx += 2;
                }
                Frame::Open(a, b) => {
                    total += a.value() + b.value();
                    idx += 2;
                }
            }
        }
        total += tenth.total();
        Some(total)
    }

    fn roll_regular(&mut self, p: Pins) -> Result<(), Error> {
        match self.pending {
            PartialFrame::Empty => {
                if p.value() == 10 {
                    self.frames.push(Frame::Strike);
                    // `pending` stays `Empty` for the next frame.
                } else {
                    self.pending = PartialFrame::Half(p);
                }
            }
            PartialFrame::Half(first) => {
                if p.value() > 10 - first.value() {
                    return Err(Error::NotEnoughPinsLeft);
                }
                if first.value() + p.value() == 10 {
                    self.frames.push(Frame::Spare(first));
                } else {
                    self.frames.push(Frame::Open(first, p));
                }
                self.pending = PartialFrame::Empty;
            }
        }
        Ok(())
    }

    fn roll_tenth(&mut self, p: Pins) -> Result<(), Error> {
        match self.tenth_pending {
            PartialTenthFrame::Empty => {
                self.tenth_pending = if p.value() == 10 {
                    PartialTenthFrame::StrikeBonus1 // rack resets to 10
                } else {
                    PartialTenthFrame::Open1(p)
                };
            }
            PartialTenthFrame::Open1(a) => {
                // a < 10; remaining = 10 - a.
                if p.value() > 10 - a.value() {
                    return Err(Error::NotEnoughPinsLeft);
                }
                if a.value() + p.value() == 10 {
                    self.tenth_pending = PartialTenthFrame::SpareBonus(a); // rack resets
                } else {
                    self.tenth = Some(TenthFrame::Open {
                        first: a,
                        second: p,
                    });
                }
            }
            PartialTenthFrame::StrikeBonus1 => {
                // Fresh rack; `p` already validated <= 10.
                self.tenth_pending = PartialTenthFrame::StrikeBonus2(p);
            }
            PartialTenthFrame::StrikeBonus2(b) => {
                // If the first bonus was a strike the rack reset; otherwise it
                // has `10 - b` pins standing.
                let remaining = if b.value() == 10 { 10 } else { 10 - b.value() };
                if p.value() > remaining {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.tenth = Some(TenthFrame::Strike {
                    bonus1: b,
                    bonus2: p,
                });
            }
            PartialTenthFrame::SpareBonus(a) => {
                // Fresh rack; `p` already validated <= 10.
                self.tenth = Some(TenthFrame::Spare { first: a, bonus: p });
            }
        }
        Ok(())
    }
}
