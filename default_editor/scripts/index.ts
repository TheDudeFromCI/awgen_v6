export async function main() {
  log("Hello, world.");

  let run = true;
  do {
    try {
      let packet = await fetchPacket();
      switch (packet["type"]) {
        case "shutdown":
          log("Shutting down...");
          run = false;
          break;

        case "count":
          log(`Count: ${packet["value"]}`);
          break;

        default:
          throw `Unknown packet: ${packet["type"]}`;
      }
    } catch (error) {
      log(`Error: ${error}`);
      run = false;
    }
  } while (run);
}
