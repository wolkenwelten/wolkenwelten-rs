const msgs = [
	`Welcome to WolkenWelten ${WWC.VERSION}!`,
	"Use WASD to move, Space to jump and Shift to sprint",
	"Use your mouse to mine/place blocks or punch crabs",
	"Nothing will be saved, as soon as you die or quit it's gone"
].reverse();
const PopMsg = () => {
	if(!msgs.length){return;}
	console.log(msgs.pop());
	setTimeout(PopMsg,3000);
}
PopMsg();

const vec_rot_shift = dir => ({
	x: -dir.z,
	y: dir.x,
	z: dir.y,
});


WWC.itemSetMesh(1,1);
WWC.itemSetAmount(1,69);