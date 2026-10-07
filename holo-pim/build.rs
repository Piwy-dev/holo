fn main() {
	let mut yang_ctx = holo_yang::new_context();
	holo_yang::load_modules(&mut yang_ctx, holo_yang::implemented_modules::PIM);
}
