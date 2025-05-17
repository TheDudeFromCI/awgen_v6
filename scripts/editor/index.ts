export async function main() {
  log("Hello, world.");

  for (let i = 0; i < 10; i++) {
    await sleep(1000);
    log(i.toString());
  }
}
