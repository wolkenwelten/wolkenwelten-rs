/***** Core Types *****/
type SfxId = number;
type BlockId = number;
type MessageType = string;

interface WWCInterface {
    getBlock:(x:number, y:number, z:number) => BlockId,
    setBlock:(x:number, y:number, z:number, block:BlockId) => void,
    sfxPlay:(x:number, y:number, z:number, volume:number, sfx:SfxId) => void,
    print:(value:any) => void,
    eprint:(value:any) => void,
	game_log:(value:any) => void,
}
declare const WWC:WWCInterface;

interface Message {
    T: MessageType,
    [propName: string]: any,
}

/***** Vec3 *****/
interface Vec3 {
    x:number,
    y:number,
    z:number,
}
const vec_new = (x:number, y:number, z:number):Vec3 => ({x,y,z});
const vec_add = (a:Vec3, b:Vec3) => ({x: a.x+b.x, y:a.y+b.y, z:a.z+b.z});
const vec_log = (pos:Vec3) => console.log(`[${pos.x}, ${pos.y}, ${pos.z}]`);

/***** EventQueue *****/
interface TimeoutQueueEntry {
    id: TimeoutQueueId,
    interval: number,
    waitUntil: number,
    cont: () => void,
}
type TimeoutQueueId = string;

const WolkenWelten = (() => {
	let curMillis = 0;
	let timeoutIds = 0;
	let timeoutQueue:Array<TimeoutQueueEntry> = [];

	const runQueue = () => {
		const old = timeoutQueue;
		timeoutQueue = [];
		const n = old.filter(e => {
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
		timeoutQueue = n.concat(timeoutQueue);
	};

	const getTimeoutId = ():TimeoutQueueId => String(++timeoutIds);
	const setTimeout = (cont:() => void, waitUntil: number) => {
		const id = getTimeoutId();
		timeoutQueue.push({id, cont, interval:0, waitUntil: waitUntil + curMillis});
	};

	const clearTimeout = (id:TimeoutQueueId) => timeoutQueue = timeoutQueue.filter(v => v.id !== id);
	const clearInterval = clearTimeout;

	const setInterval = (cont:() => void, interval:number) => {
		const id = getTimeoutId();
		const waitUntil = curMillis + interval;
		timeoutQueue.push({id, cont, waitUntil, interval});
	}

	let msgHandler:Map<MessageType, Array<(msg:Message) => void>> = new Map();

	const dispatch = (msg:Message) => {
		let handler = msgHandler.get(msg.T);
		if(handler){
			for(const h of handler){
				h(msg);
			}
		}
	};

	const addMsgHandler = (T:MessageType, λ:(msg:Message) => void) => {
		let h = msgHandler.get(T);
		if(h){
			h.push(λ);
		} else {
			msgHandler.set(T,[λ]);
		}
	}

	const removeMsgHandler = (T:MessageType, λ:(msg:Message) => void) => {
		let h = msgHandler.get(T);
		if(h){
			msgHandler.set(T, h.filter(l => l !== λ))
		}
	}

	const clearMsgHandler = () => {
		msgHandler.clear();
	}

	const tick = (ticks:number, msgs:Array<Message>) => {
		curMillis = ticks;
		for(const msg of msgs){
			dispatch(msg);
		}
		runQueue();
	};

	const log = (value:any) => {
		const v = value.toString();
		WWC.print(v+"\n");
		WWC.game_log(v);
	}
	const error = (value:any) => WWC.eprint(value.toString()+"\n");

	return {
		tick,
		setTimeout,
		clearTimeout,
		setInterval,
		clearInterval,
		addMsgHandler,
		clearMsgHandler,
		removeMsgHandler,
		error,
		log
	};
})();

const setTimeout = WolkenWelten.setTimeout;
const clearTimeout = WolkenWelten.clearTimeout;
const setInterval = WolkenWelten.setInterval;
const clearInterval = WolkenWelten.clearInterval;
const setImmediate = (cont:() => void) => setTimeout(cont, 0);
const console = {
	log: WolkenWelten.log,
	error: WolkenWelten.error
};

const WW = {
	getBlock: (p:Vec3) => WWC.getBlock(p.x, p.y, p.z),
	setBlock: (p:Vec3, b:BlockId) => WWC.setBlock(p.x, p.y, p.z, b),
	sfxPlay: (p:Vec3, v:number, sfx:SfxId) => WWC.sfxPlay(p.x, p.y, p.z, v, sfx),
	sfx: {
		jump: 1,
		hook_fire: 2,
		ungh: 3,
		step: 4,
		stomp: 5,
		bomb: 6,
		pock: 7,
		tock: 8
	},
	block: {
		air: 0,
		dirt: 1,
		grass: 2,
		stone: 3,
		coal: 4,
		spruceLog: 5,
		spruceLeaves: 6,
		dryGrass: 7,
		roots: 8,
		obsidian: 9,
		oakLog: 10,
		oakLeaves: 11,
		hematite: 12,
		marbleBlock: 13,
		marblePillar: 14,
		marbleBlocks: 15,
		acaciaLeaves: 16,
		boards: 17,
		crystals: 18,
		sakuraLeaves: 19,
		birchLog: 20,
		flowerBush: 21,
		dateBush: 22,
	},
	addMsgHandler: WolkenWelten.addMsgHandler,
	clearMsgHandler: WolkenWelten.clearMsgHandler,
	removeMsgHandler: WolkenWelten.removeMsgHandler,
};
//WW.addMsgHandler("BlockBreak", m => m.block === WW.block.dirt && WW.sfxPlay(m.pos, 1.0, WW.sfx.bomb));

"WW Ready!"