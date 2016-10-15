macro_rules! shape {
    ($([$x:expr, $y:expr], [$tex_x:expr, $tex_y:expr]),*) => {{
        [$(
            $crate::hsgraphics::gfx2d::Vertex::new([$x, $y], [$tex_x, $tex_y]),
         )*]
    }};
}
