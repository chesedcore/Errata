Class reference so far:

```gdscript
#comments are either // or #, you can use anything!

#annotations are indicated with "@anno" decorator syntax, same as in gdscript.
@icon("res://icon.svg")

#class_names are the same as in gdscript. leave empty to make it anonymous.
class_name NewClass

#inheritance. leave empty to make it auto-inherit from RefCounted
extends BaseClass

#type inferrence is *forced by default.* this forces `a` to be typed to 3.
var a = 3
#forces `s` to be a `String`
var s = "this is a string"

#dynamic typing is an opt-in feature, because strong typing is the default.
#newlines are significant and represent a new statement.
var any_type: Variant = "literally anything"
any_type = 3

#constant expressions ("constexpr") are denoted with `const`. they are always immutable.
const ANSWER = 42

#simple enumerations, same as in gdscript.
enum Number {
  ZERO = 0,
  ONE,
  TWO,
}

#anonymous enumerations like in gdscript *do not exist,* and you must name your enums.

#the enum can also double as a tagged unions, basically enums that can contain a value attached to them.
enum Shape {
  CIRCLE(radius: float),
  SQUARE(side: float),
  RECT(x_side: float, y_side: float),
}

var circle = Shape.CIRCLE(3)
if circle is Shape.CIRCLE(radius) {
  print(radius) #prints 3
}

#vectors are the exact same as in gdscript.
var v2 = Vector2(1, 2)
var v3 = Vector3(3, 4, 5)

#functions must define their scope within braces.
func do_nothing() {
  #this function does nothing, and 'pass' is not used.
}

#if taking in parameters, you MUST state the argument types. explicitly state "Variant" for untypedness.
func print_sum(a1: int, a2: int, throwaway: Variant) {
  print(a1 + a2)
  print(throwaway)
}

#if returning a value, you MUST state the return type.
func return_the_same_thing(x: int) -> int {
    return x
}

#if the function would change the state of a global variable that is outside its scope, that function must be marked as 'mut'
var count = 3
mut func increment_count() {
#^^^^^^ marked as mut because count is outside of the local scope.
    count += 1
        #^^^^ state changed here
}

#if the function would change the state of an argument, that argument must be marked as 'mut'
func heal_player(mut player: Player, amount: int) {
                #^^^ because player is mutated in the next line, while amount is read-only
    player.health += amount
         #^^^^^^^^^^^^^^^^^ state changed here
}

#instead of 'class' for inner class, it has been repurposed to 'struct' to remove ambiguity.
#all scopes are, as demonstrated before, marked with braces{}.
struct InnerStruct {
    var counter: int

    func read_counter() -> int {
        return self.counter
    }

    mut func increment() { #methods must obey the same mutation rules
        self.counter += 1
    }
}



```
