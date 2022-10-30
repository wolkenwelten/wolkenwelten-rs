interface IDeno {
    core: {
      print: (_val:string) => void,
    }
  }
  declare const Deno:IDeno;

  interface IQueueEntries {
    id: string,
    cont: () => void,
    waitUntil: number,
    interval: number,
  }

  const WolkenWelten = (() => {
      let curMillis = 0;
      let timeoutIds = 0;
      let timeoutQueue:IQueueEntries[] = [];

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

      const getTimeoutId = ():string => String(++timeoutIds);

      const setTimeout = (cont:() => void, waitUntil:number) => {
        const id = getTimeoutId();
        timeoutQueue.push({id, cont, interval:0, waitUntil});
      };

      const clearTimeout = (id:string) => {
        timeoutQueue.filter(v => v.id !== id);
      };
      const clearInterval = clearTimeout;

      const setInterval = (cont:() => void, interval:number) => {
        const id = getTimeoutId();
        const waitUntil = curMillis + interval;
        timeoutQueue.push({id, cont, waitUntil, interval});
      }

      const tick = (ticks:number) => {
        curMillis = ticks;
        runQueue();
      };

      const log = (value:any) => {
        Deno.core.print(value.toString()+"\n");
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
  const setImmediate = (cont:() => void) => setTimeout(cont, 0);
  const console = {
    log: WolkenWelten.log
  };

console.log("V8 Ready!");

(() => {
    let seconds = 0;
    setInterval(() => {
        const secs = String(++seconds);
        const s = seconds > 1 ? "s" : "";
        console.log(secs + " second" + s + " have elapsed.");
    }, 1000);
})();