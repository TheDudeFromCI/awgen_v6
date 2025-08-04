export async function main() {
  let initPacket = (await fetchPacket()) as PacketFromClient.Init;
  log(`Received initialization packet: ${JSON.stringify(initPacket)}`);

  sendPackets({
    type: "init",
    name: "Default Editor",
    version: [0, 0, 1],
  });

  let run = true;
  while (run) {
    try {
      let packet = await fetchPacket();
      switch (packet["type"]) {
        case "init":
          log("Initialization packet received.");
          break;
        case "shutdown":
          log("Shutting down...");
          run = false;
          break;
      }
    } catch (error) {
      log(`Error: ${error}`);
      run = false;
    }
  }
}
