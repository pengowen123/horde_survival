#[derive(Clone, Copy)]
struct Animation {
    post: usize,
    timer: usize,
    state: AnimationState,
    casting: bool,
}

#[derive(Clone, Copy, Debug)]
enum AnimationState {
    Pre,
    Post,
}

#[derive(Clone)]
pub struct AnimationList {
    list: [Animation; 5],
}

impl Animation {
    pub fn new() -> Animation {
        Animation {
            post: 0,
            timer: 0,
            state: AnimationState::Pre,
            casting: false,
        }
    }

    pub fn update(&mut self) {
        self.casting = false;

        match self.state {
            AnimationState::Pre => {
                if self.timer == 1 {
                    self.timer = self.post;
                    self.state = AnimationState::Post;
                    self.casting = true;
                }
            },
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

    pub fn can_cast(&self) -> bool {
        match self.state {
            AnimationState::Pre => self.timer == 0,
            AnimationState::Post => false,
        }
    }

    pub fn start(&mut self, pre: usize, post: usize) {
        self.timer = pre + 1;
        self.post = post;
    }

    pub fn is_casting(&self) -> bool {
        self.casting
    }
}

impl AnimationList {
    pub fn new() -> AnimationList {
        AnimationList {
            list: [Animation::new(); 5],
        }
    }

    pub fn update(&mut self) {
        for animation in &mut self.list {
            animation.update();
        }
    }

    pub fn can_cast(&self, id: usize) -> bool {
        self.list[id].can_cast()
    }

    pub fn can_attack(&self) -> bool {
        self.list[0].can_cast()
    }

    pub fn start(&mut self, id: usize, pre: usize, post: usize) {
        self.list[id].start(pre, post);
    }

    pub fn is_casting(&self, id: usize) -> bool {
        self.list[id].is_casting()
    }
}
