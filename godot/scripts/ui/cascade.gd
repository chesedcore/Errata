##a node over-engineered to work as a UI animation tool, tweening its children
##in and out of view.
class_name Cascade extends Control

##if this array has [i]any[/i] nodes in it, then instead of this node's direct
##children, those nodes are tweened instead. if using a marker2d to denote node
##starting position, make sure that marker2d has the same parent as those nodes.
##this is because this tool only uses [code]position[/code] in consideration
##for tweening and not [code]global_position[/code].
@export var use_these_nodes_instead: Array[Node]

##does what it says on the tin. these nodes will no longer be part of the animations.
@export var exclude_these_nodes: Array[Node]

##what animation type to deploy for this cascade.
@export var cascade_type: CASCADE_TYPE

##if on, the tween-in animation occurs when this node is ready.
@export var begin_on_ready: bool = false

##where exactly the items should be placed during the start state.
@export var start_marker: Marker2D

@export_subgroup("Tween Settings")
#region tween settings
##the time each individual animation takes. 
##this is NOT the cumulative animation time.
@export var tween_in_time: float = 0.6

##time it takes between two tweens to fire off.
@export var stagger_time: float = 0.1
##the easing used per tween.
@export var easing := Tween.EASE_OUT
##the transition used per tween.
@export var trans := Tween.TRANS_EXPO
#endregion

#@export_subgroup("Additional Behaviour Modifiers")
##region behaviour mod settings
#@export var tween_alpha: bool ----> ##UNIMPLEMENTED
##endregion

enum FADE {
	
	##elements fly into the screen.
	FADE_IN,
	
	##elements fly out of the screen.
	FADE_OUT,
	
}

signal cascade_started(fade: FADE)
signal cascade_finished(fade: FADE)
signal last_cascade_started(fade: FADE)

##the dictionary that holds every node while corresponding it to its original
##position. intended to act as this tool's single source of truth.
##note the positions here are the nodes' [code]position[/code] 
##variable and NOT [code]global_position[/code].
##call [code]prepare_positions_arr()[/code] to refresh its state.
var _original_positions: Dictionary[Node, Vector2]

enum CASCADE_TYPE {
	
	##the units are tweened based on the order they appear in get_children().
	##if using [code]use_these_nodes_instead[/code], then the nodes are tweened
	##based on their indices in the array. uses [code]start_marker[/code]
	##to determine where to start position tweening from.
	ORDERED,
	
	##the units are tweened randomly. also uses [code]start_marker[/code].
	RANDOM,
	
	##the units are first squashed into Vec2(0,-2) then scattered
	##randomly along the x-axis. then they are tweened into position.
	##tween-outs involve the children shrinking away while being scattered
	##along the x-axis based on the sine of their index in the array they are
	##obtained from. this cascade type ignores several custom settings.
	##ignores [code]start_marker[/code].
	##Inspired from Hipxel's HIT-ZONE game menu.
	HITZONE,
	
	##exactly the same as hitzone, but the fly-in fixes the start position.x
	##of units in accordance to the sine of their indices. try both and see
	##what you're into more.
	SINE_ORDERED_HITZONE
}

#region preparation for tweening

##empties the original positions array, then assigns new positions to it
##based on [code]use_these_nodes_instead[/code]. intended to be called when
##the state of the cascade is supposed to be reset. 
func prepare_positions_arr() -> void:
	_original_positions.clear()
	if use_these_nodes_instead: _prepare_targets_instead()
	else: _prepare_children()

func _prepare_children() -> void:
	for node in get_children():
		if node is Marker2D or node is Timer: continue
		_original_positions[node] = node.position

func _prepare_targets_instead() -> void:
	for node in use_these_nodes_instead:
		_original_positions[node] = node.position

##a function that gets all the units that are supposed to take part in animation.
##uses [code]_original_positions[/code] as the basis of its truth, so it can be 
##reliably used immediately after a state change too. applies the exclusion filter
##marked in [code]exclude_these_nodes[/code].
func get_units() -> Array[Node]:
	var units := _original_positions.keys()
	for target in exclude_these_nodes: units.erase(target)
	return units

##does what it says on the damned tin. requires a [code]start_marker[/code]
##to operate. obviously. uses the marker's [code]position[/code] instead of
##[code]global_position[/code], so you better make sure all the units 
##being tweened, and the marker, have a common parent.
func place_units_at_marker() -> void:
	assert(start_marker, "...you forgot to set a start marker, dumbass.")
	for unit in get_units():
		unit.position = start_marker.position

##ignores the [code]start_marker[/code] and scatters the units with a left
##leaning bias, while also squashing their scale into Vector2(0, -2)
##this makes them effectively invisible on the 2D plane.
func hitzone_init() -> void:
	for unit in get_units():
		unit.position += Vector2(randf_range(-255,-55), randf_range(-55,55))
		unit.scale = Vector2(0, -2)

##immediately set the units to the state they would be in before their
##tween-in animation.
func set_start_state() -> void:
	match cascade_type:
		CASCADE_TYPE.ORDERED, CASCADE_TYPE.RANDOM:
			place_units_at_marker()
		CASCADE_TYPE.HITZONE:
			hitzone_init()

#endregion

#region main animation engine
var tween: Tween
func _cascade_engine(
	filter: Callable, 
	modifiers: Array[Callable],
	reverse: bool = false
) -> void:
	
	var fade_status := FADE.FADE_IN
	if reverse: fade_status = FADE.FADE_OUT
	
	cascade_started.emit(fade_status)
	var units: Array[Node] = filter.call(get_units())
	if not units:
		last_cascade_started.emit(fade_status)
		cascade_finished.emit(fade_status)
		return
	
	if tween: tween.kill()
	tween = create_tween().set_ease(easing).set_trans(trans)
	tween.set_parallel(true)
	
	var cumulative_delay: float = 0.0
	for unit in units:
		for modifier in modifiers: 
			modifier.call(tween, unit, cumulative_delay, reverse)
		cumulative_delay += stagger_time
	
	tween.finished.connect(cascade_finished.emit.bind(fade_status))
	tween.tween_callback(last_cascade_started.emit.bind(fade_status))\
	.set_delay(maxf(cumulative_delay - stagger_time, 0.0))

#endregion

#region filters

#signature:
#filter(arr: Array[Node]) -> Array[Node]

##do absolutely fucking nothing. useful for cascade types that retain 
##natural order, like ORDERED or HITZONE.
func filter_nothing(arr: Array[Node]) -> Array[Node]: 
	return arr

##shuffle the units in the array.
func filter_random(arr: Array[Node]) -> Array[Node]:
	arr.shuffle()
	return arr

##scatter the units around, squish them into Vec(0,-2).
func filter_hitzone(arr: Array[Node]) -> Array[Node]:
	for unit in arr:
		var original_position := _original_positions[unit]
		unit.position = original_position + \
						Vector2(randf_range(-255, 55), randf_range(-55,55))
		unit.scale = Vector2(0, -2)
	return arr

##hitzone leaves everything in a haphazard state. this resets them back.
func hitzone_reset_pos_for_next_in() -> void:
	for unit in get_children():
		if unit is Marker2D or unit is Timer: continue
		var original_pos := _original_positions[unit]
		unit.position = original_pos


##scatter the units from hitzone into a sine-ordered manner.
func filter_sine_hitzone(arr: Array[Node]) -> Array[Node]:
	#arr.sort_custom(
		#func(thing: Node, thing2: Node) -> bool:
			#return thing.position.y < thing2.position.y
	#)
	for idx in arr.size():
		var unit := arr[idx]
		var original_position := _original_positions[unit]
		unit.position = original_position
		unit.scale = Vector2(0, -2)
		unit.position.x += _original_positions[unit].x + ((sin(idx) * 100) - unit.position.x * 0.1)
	return arr
#endregion

#region modifiers

#signature:
#modifier(tween: Tween, node: Node, delay: float, rev: bool) -> void

func mod_marker_pos(t: Tween, node: Node, delay: float, rev: bool) -> void:
	
	assert(start_marker, "...you forgot the start marker yet fucking again.")
	var target_position: Vector2
	if rev: target_position = start_marker.position
	else: target_position = _original_positions[node]
	
	t.tween_property(node, "position", target_position, tween_in_time)\
	.set_delay(delay)

func mod_hitzone_in(t: Tween, node: Node, delay: float, _r: bool) -> void:
	assert(not _r, "This func only supports a reverse val of true.")
	var target_position: Vector2 = _original_positions[node]
	t.tween_property(node, "position", target_position, tween_in_time / 2.)\
	.set_delay(delay)
	t.tween_property(node, "scale", Vector2(1,1), tween_in_time)\
	.set_delay(delay)

func mod_hitzone_out(t: Tween, node: Node, _delay: float, _rev: bool) -> void:
	var pos := _original_positions[node]
	var target_pos := pos + Vector2(sin(pos.x) * 100, randf_range(-30, 30))
	t.tween_property(node, "position", target_pos, maxf(tween_in_time / 2, 0.5))
	t.tween_property(node, "scale", Vector2(-2, 0), maxf(tween_in_time / 1.5, 0.5))

#endregion

#region cascade methods
func _cascade_in_ordered() -> void:
	_cascade_engine(filter_nothing, [mod_marker_pos])

func _cascade_out_ordered() -> void:
	_cascade_engine(filter_nothing, [mod_marker_pos], true)

func _cascade_in_random() -> void:
	_cascade_engine(filter_random, [mod_marker_pos])

func _cascade_out_random() -> void:
	_cascade_engine(filter_random, [mod_marker_pos], true)

func _cascade_in_hitzone() -> void:
	_cascade_engine(filter_hitzone, [mod_hitzone_in])

func _cascade_out_hitzone() -> void:
	_cascade_engine(filter_nothing, [mod_hitzone_out], true)

func _cascade_in_sine_hitzone() -> void:
	_cascade_engine(filter_sine_hitzone, [mod_hitzone_in])
#endregion

#region cascade starters
##the public API method intended to start off the entire cascade chain.
##the UI elements fly into view from oblivion.
func cascade_in() -> void:
	match cascade_type:
		CASCADE_TYPE.ORDERED:
			_cascade_in_ordered()
		CASCADE_TYPE.RANDOM:
			_cascade_in_random()
		CASCADE_TYPE.HITZONE:
			hitzone_reset_pos_for_next_in()
			_cascade_in_hitzone()
		CASCADE_TYPE.SINE_ORDERED_HITZONE:
			_cascade_in_sine_hitzone()

##the public API method intended to block off the entire cascade chain.
##the UI elements fly out of view into the ragnarok.
##optionally, you can add a flag to ignore excluded nodes,
##so that every-single-fucking-thing flies out regardless of state.
func cascade_out(ignore_exclusions: bool = false) -> void:
	var saved_exclusions := exclude_these_nodes.duplicate()
	if ignore_exclusions: exclude_these_nodes.clear()
	
	match cascade_type:
		CASCADE_TYPE.ORDERED:
			_cascade_out_ordered()
		CASCADE_TYPE.HITZONE, CASCADE_TYPE.SINE_ORDERED_HITZONE:
			_cascade_out_hitzone()
		CASCADE_TYPE.RANDOM:
			_cascade_out_random()
	
	if ignore_exclusions: exclude_these_nodes = saved_exclusions
#endregion

func _ready() -> void:
	await get_tree().process_frame
	prepare_positions_arr()
	await get_tree().process_frame
	set_start_state()
	if begin_on_ready: cascade_in()
