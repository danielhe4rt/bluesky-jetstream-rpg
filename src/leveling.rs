use atrium_api::app::bsky::actor::defs::ProfileViewDetailed;

/// The maximum level a user can attain.
pub const LEVEL_CAP: i32 = 100000;

/// How many XP are required for each level after the base.
pub const EXPERIENCE_PER_LEVEL: i32 = 100;

/// How many XP you need to reach Level 1.
pub const BASE_EXPERIENCE: i32 = 50;

pub const POST_EVENT_XP: i32 = 30;

/// A response struct that includes additional metadata about leveling.
#[derive(Debug, Default, Clone)]
pub struct LevelResponse {
    /// The new total level (clamped at LEVEL_CAP).
    pub level: i32,
    /// The user's total, accumulated experience points.
    pub experience: i32,
    /// The XP required to reach the *next* level. Zero if already at or above `LEVEL_CAP`.
    pub experience_to_next_level: i32,
    /// How many levels the user gained in this single experience increment.
    pub _levels_gained: i32,
    /// A float (0.0 ..= 1.0) representing how far the user is from `level` to `level + 1`.
    /// If `level` = `LEVEL_CAP`, this can safely be 1.0 (or 0.0, depending on your preference).
    pub _progress_percentage: f32,
}

/// Calculate new level/XP state, given the current XP and newly gained XP.
/// This uses a simple *arithmetic progression* for XP thresholds.
///
/// # Arithmetic Progression
/// - `xp_for_level(1) = BASE_EXPERIENCE`
/// - `xp_for_level(n) = BASE_EXPERIENCE + (n-1)*EXPERIENCE_PER_LEVEL`
///
/// # Examples
/// ```
/// let resp = calculate_experience(0, 30);
/// // resp.level == 1
/// // resp.experience == 30
/// // resp.experience_to_next_level == 20  (need total of 50 to hit level 1 threshold)
/// // resp.levels_gained == 0
/// // resp.progress_percentage == 0.6  (30 / 50)
/// ```
pub fn calculate_experience(current_experience: i32, new_experience: i32) -> LevelResponse {
    // Sum up total XP so far.

    // Figure out old level vs. new level to track "levels gained."
    let old_level = get_level_from_xp(current_experience);
    let new_level = get_level_from_xp(new_experience);

    let level = new_level.min(LEVEL_CAP);

    // If we're at or above the level cap, no more progress to be made.
    let (experience_to_next_level, progress_percentage) = if level >= LEVEL_CAP {
        (0, 1.0_f32)
    } else {
        // The XP needed to *reach* the next level.
        let next_level_xp = xp_for_level(level + 1);

        // For progress percentage, we see:
        //  current_level_xp = xp_for_level(level)
        //  next_level_xp = xp_for_level(level + 1)
        //  range = next_level_xp - current_level_xp
        //  progress = (total_experience - current_level_xp) / range
        let current_level_xp = xp_for_level(level);
        let range = next_level_xp.saturating_sub(current_level_xp).max(1);
        let progress = (new_experience.saturating_sub(current_level_xp)) as f32 / range as f32;

        (next_level_xp, progress)
    };

    // How many levels were gained in this single increment?
    let _levels_gained = (new_level - old_level).max(0);

    LevelResponse {
        level,
        experience: new_experience,
        experience_to_next_level,
        _levels_gained,
        _progress_percentage: progress_percentage,
    }
}

//
// ─────────────────────────────────────────────────────────────────────
//    HELPER FUNCTIONS
// ─────────────────────────────────────────────────────────────────────
//

/// Returns the XP required to *reach* `level`.
/// By definition:
///     xp_for_level(1) = BASE_EXPERIENCE
///     xp_for_level(n) = BASE_EXPERIENCE + (n-1)*EXPERIENCE_PER_LEVEL
///
/// # Note:
/// If you want a *non-linear* progression, modify the formula here.
/// e.g. for a *logarithmic progression*:
///     xp_for_level(n) = A * ln(n) + B
/// or for an *exponential progression*:
///     xp_for_level(n) = base_exp * (factor ^ (n - 1)).
pub fn xp_for_level(level: i32) -> i32 {
    if level <= 1 {
        BASE_EXPERIENCE
    } else {
        BASE_EXPERIENCE + (level - 1) * EXPERIENCE_PER_LEVEL
    }
}

/// Given a total XP value, compute which level you're on (without clamping).
/// This is the inverse of `xp_for_level`.
///
/// # Example (Arithmetic):
/// Level n means:
///     xp_for_level(n) <= xp < xp_for_level(n+1)
///
pub fn get_level_from_xp(xp: i32) -> i32 {
    // If you have less XP than `BASE_EXPERIENCE`, you haven't fully
    // reached level 1 yet—but let's call it level 1 for convenience.
    if xp < BASE_EXPERIENCE {
        return 1;
    }

    // Solve for n in:
    //   xp >= xp_for_level(n) = BASE_EXPERIENCE + (n-1)*EXPERIENCE_PER_LEVEL
    //   => xp - BASE_EXPERIENCE >= (n-1)*EXPERIENCE_PER_LEVEL
    //   => (n - 1) <= (xp - BASE_EXPERIENCE) / EXPERIENCE_PER_LEVEL
    //   => n <= 1 + ((xp - BASE_EXPERIENCE) / EXPERIENCE_PER_LEVEL)
    //
    // We take the integer floor of that expression.
    1 + (xp - BASE_EXPERIENCE) / EXPERIENCE_PER_LEVEL
}

/// Calculate the base level of a user based on their Bsky profile
///
/// # Arguments
///
/// * `profile`: &ProfileViewDetailed - The profile to calculate the base level from.
///
/// returns: LevelResponse
pub fn get_base_level_from_bsky_profile(profile: &ProfileViewDetailed) -> LevelResponse {
    // TODO: implement a way to list all likes sent by an account.
    let experience = profile.posts_count.unwrap() as i32 * POST_EVENT_XP;

    calculate_experience(0, experience)
}
