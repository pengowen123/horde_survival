/// An animation
#[derive(Clone, Copy, Default)]
struct Animation {
    post: usize,
    timer: usize,
    state: AnimationState,
    casting: bool,
}

/// The state of an animation
/// A Pre animation state means the animation that plays before the action occurs
/// A Post animation state means the animation that plays after the action occurs
#[derive(Clone, Copy, Debug)]
enum AnimationState {
    Pre,
    Post,
}

/// A list of animations representing the full set of animations an entity can have
#[derive(Clone, Default)]
pub struct AnimationList {
    list: [Animation; 5],
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Pre
    }
}

impl Animation {
    /// Updates the animation
    pub fn update(&mut self) {
        self.casting = false;

        match self.state {
            AnimationState::Pre => {
                if self.timer == 1 {
                    self.timer = self.post;
                    self.state = AnimationState::Post;
                    self.casting = true;
                }
            }
            AnimationState::Post => {
                if self.timer == 0 {
                    self.state = AnimationState::Pre;
                    self.casting = false;
                }
            }
        }

        if self.timer > 0 {
            self.timer -= 1;
        }
    }

    /// Returns whether the animation is finished
    pub fn is_finished(&self) -> bool {
        match self.state {
            AnimationState::Pre => self.timer == 0,
            AnimationState::Post => false,
        }
    }

    /// Starts the animation with the given timers
    pub fn start(&mut self, pre: usize, post: usize) {
        self.timer = pre + 1;
        self.post = post;
    }

    /// Returns whether the animation is playing
    /// Returns false if the current state is post-animation (if the action has already happened)
    pub fn is_playing(&self) -> bool {
        self.casting
    }
}

impl AnimationList {
    /// Updates all animations
    // NOTE: Animation 0 is used for attacking, so ability 0 is animation 1
    //       Example:
    //
    //       animations.is_casting(1) // first ability
    pub fn update(&mut self) {
        for animation in &mut self.list {
            animation.update();
        }
    }

    /// Returns whether the animation with the ID is finished
    pub fn is_finished(&self, id: usize) -> bool {
        self.list[id].is_finished()
    }

    /// Returns whether the attack animation is finished
    pub fn can_attack(&self) -> bool {
        self.is_finished(0)
    }

    /// Returns whether the attack animation is playing
    pub fn is_attacking(&self) -> bool {
        self.is_playing(0)
    }

    /// Starts an animation
    pub fn start(&mut self, id: usize, pre: usize, post: usize) {
        self.list[id].start(pre, post);
    }

    /// Returns whether the animation with the ID is playing
    pub fn is_playing(&self, id: usize) -> bool {
        self.list[id].is_playing()
    }
}
