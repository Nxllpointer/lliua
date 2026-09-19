#import "@preview/pinit:0.2.2": pin, pinit-arrow, pinit-point-to, pinit-point-from, pinit-line

#set page(height: auto)

#let draw-counter = counter("draw-counter")

#let draw(stack) = context {
  set align(center)
  draw-counter.update(i => i + 1)
  let drawi = str(draw-counter.get().first())
  
  grid(
    rows: 1,
     columns: stack.arena_mem.len(),
     stroke: black,
     ..stack.arena_mem.enumerate().map(((i, m)) => {
       grid.cell(
         fill: m.at("color"),
         block(width: 2em, height: 2em)[
           #set align(center + horizon)
           #set par(spacing: 0pt)
           #pin(drawi + "a-" + str(i))
           #block[#i]
         ]
       )
    })
  )

  [ \ \ ]
  
  if stack.vstack.len() > 0 {
    grid(
      rows: 1,
       columns: stack.vstack.len(),
       stroke: black,
       ..stack.vstack.enumerate().map(((i, v)) => {
         grid.cell(fill: v.color, block(width: 2em, height: 2em)[
           #set align(center + horizon)
           #set par(spacing: 0pt)
           #pin(drawi + "v-" + str(i))
           #block[#i]
         ])
      })
    )
  }
  else {
    block[*Empty stack*]
  }

  pinit-point-from(
    drawi + "a-" + str(stack.base_ptr),
    pin-dx: 0pt,
    pin-dy: 0pt,
    offset-dx: 0pt,
    offset-dy: -25pt
  )[]

  
  for (i, v) in stack.vstack.enumerate() {
    pinit-arrow(
      drawi + "v-" + str(i),
      drawi + "a-" + str(v.ptr),
      start-dy: -5pt,
      end-dy: 10pt,
    )
  }


  line(length: 100%)
}

#let alloc(stack, n, color) = {
    let start = stack.base_ptr
    stack.base_ptr += n
  
    for i in range(start, stack.base_ptr) {
      stack.arena_mem.at(i).color = color
    }
    
    return (stack, start)
}

#let push(stack, ptr, color) = {
  stack.vstack.push((ptr: ptr, color: color))
  return stack
}

#let pop(stack) = {
  stack.vstack.pop()
  return stack
}

#let stack_state = state("stack", (
  vstack: (),
  arena_mem: ((color: white,),) * 20,
  base_ptr: 0
))

#text(size: 20pt)[
  $ ⟜ plus med 2 times 10 med #emoji.die $
```
&uasm.root = (
    ⚂,
    push 10,
    ×,
    push 2,
    ⟜(
        +,
    ),
)
```
]

\ \ \


#{
  let stack = (
    vstack: (),
    arena_mem: ((color: white,),) * 20,
    base_ptr: 0
  )

  draw(stack)

  text(size: 20pt)[Random]
  let (stack, a_ptr) = alloc(stack, 3, red)
  stack = push(stack, a_ptr, red)
  draw(stack)
  
  text(size: 20pt)[Push 10]
  let (stack, b_ptr) = alloc(stack, 3, blue)
  stack = push(stack, b_ptr, blue)
  draw(stack)
  
  text(size: 21pt)[Multiply]
  let (stack, c_ptr) = alloc(stack, 3, green)
  stack = pop(stack)
  stack = pop(stack)
  stack = push(stack, c_ptr, green)
  draw(stack)
  
  text(size: 21pt)[Push 2]
  let (stack, d_ptr) = alloc(stack, 3, orange)
  stack = push(stack, d_ptr, orange)
  draw(stack)
  
  text(size: 21pt)[Add]
  let (stack, e_ptr) = alloc(stack, 3, color.fuchsia)
  stack = pop(stack)
  stack = pop(stack)
  stack = push(stack, e_ptr, green)
  draw(stack)
  
  text(size: 21pt)[On (Call but keep first argument before outputs)]
  stack = push(stack, d_ptr, orange)
  draw(stack)
}


