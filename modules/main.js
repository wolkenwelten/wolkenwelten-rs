console.log("V8 Ready!");

const vec_rot_shift = dir => ({
	x: -dir.z,
	y: dir.x,
	z: dir.y,
});

(() => {
	let seconds = 0;
	setInterval(() => {
		const secs = String(++seconds);
		const s = seconds > 1 ? "s" : "";
		console.log(`${secs} second${s} passed in V8 land`);
	}, 1000);
})();

let sneks = [];
const snek_tick = ({pos, dir, b, food}) => {
	const n_pos = vec_add(pos, dir);
	if(WW.get_block(n_pos) === 0){
		WW.set_block(pos, b);
		snek(n_pos, dir, b, food-1);
	} else {
		const nb = Math.max(1,(b+1) % 14);
		if(dir.x == 0){
			const adir = vec_new(-1,0,0);
			const bdir = vec_new(1,0,0);
			snek(vec_add(adir,pos), adir, nb);
			snek(vec_add(bdir,pos), bdir, nb);
		}
		if(dir.y == 0){
			const adir = vec_new(0,-1,0);
			const bdir = vec_new(0,1,0);
			snek(vec_add(adir,pos), adir, nb);
			snek(vec_add(bdir,pos), bdir, nb);
		}
		if(dir.z == 0){
			const adir = vec_new(0,0,-1);
			const bdir = vec_new(0,0,1);
			snek(vec_add(adir,pos), adir, nb);
			snek(vec_add(bdir,pos), bdir, nb);
		}
	}
}
const snek = (pos, dir, b, food=128) => {
	if(sneks.length > 4096) { return; }
	if(sneks.length > 128 && WW.get_block(pos) !== 0){ return; }
	sneks.push({pos, dir, b, food});
};

/*
(() => {
	let pos = {x:-64, y:-16, z:360};
	let dir = {x:0, y:-1, z:0};
	let b = 1;
	snek(pos,dir,b,32);
	setInterval(() => {
		const oldSneks = sneks;
		sneks = [];
		oldSneks.forEach(s => snek_tick(s));
	}, 200);
})();
*/
