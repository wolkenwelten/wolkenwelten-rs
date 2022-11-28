const WolkenWelten = (() => {
	let curMillis = 0;
	let timeoutIds = 0;
	let timeoutQueue = [];

	const runQueue = () => {
		timeoutQueue = timeoutQueue.filter(e => {
			if (curMillis >= e.waitUntil) {
				e.cont();
				if(e.interval){
					e.waitUntil = curMillis + e.interval;
				}else{
					return false;
				}
			}
			return true;
		});
	};

	const getTimeoutId = () => String(++timeoutIds);

	const setTimeout = (cont, waitUntil) => {
		const id = getTimeoutId();
		timeoutQueue.push({id, cont, interval:0, waitUntil});
	};

	const clearTimeout = (id) => {
		timeoutQueue.filter(v => v.id !== id);
	};
	const clearInterval = clearTimeout;

	const setInterval = (cont, interval) => {
		const id = getTimeoutId();
		const waitUntil = curMillis + interval;
		timeoutQueue.push({id, cont, waitUntil, interval});
	}

	const tick = (ticks) => {
		curMillis = ticks;
		runQueue();
	};

	const log = (value) => {
		WWC.print(value.toString()+"\n");
	}

	return {
		tick,
		setTimeout,
		clearTimeout,
		setInterval,
		clearInterval,
		log
	};
})();

const setTimeout = WolkenWelten.setTimeout;
const clearTimeout = WolkenWelten.clearTimeout;
const setInterval = WolkenWelten.setInterval;
const clearInterval = WolkenWelten.clearInterval;
const setImmediate = (cont) => setTimeout(cont, 0);
const console = {
	log: WolkenWelten.log
};

const vec_add = (a,b) => ({x: a.x+b.x, y:a.y+b.y, z:a.z+b.z});
const WW = {
	get_block: p => WWC.get_block(p.x, p.y, p.z),
	set_block: (p,b) => WWC.set_block(p.x, p.y, p.z, b),
};

const vec_new = (x,y,z) => ({x,y,z});

"WW Ready!"