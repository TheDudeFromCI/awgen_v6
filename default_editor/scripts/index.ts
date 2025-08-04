export async function main() {
  let run = true;
  while (run) {
    try {
      let packet = await fetchPacket();
      switch (packet["type"]) {
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
