import { Corolla } from "../js_api/index";
import { $, ShellPromise, type Subprocess } from "bun";
import { afterAll, beforeAll, expect, test } from "bun:test";

let test_server: Subprocess | undefined;
let pid: string | undefined;
const uuid = crypto.randomUUID();

beforeAll(async () => {
  await $`rm -rf ${import.meta.dir}/../tmp`;
  await $`mkdir -p ${import.meta.dir}/../tmp`;
  test_server = Bun.spawn([
    "cargo",
    "run",
    "--",
    "-s",
    "examples/example_spec.json",
    "-d",
    `tmp/${uuid}.sqlite3`,
    "-r",
    "/test",
    "--pid-file",
    `tmp/${uuid}`,
  ], { cwd: `${import.meta.dir}/..` });
  await Bun.sleep(5000);
});

afterAll(async () => {
  const pid = await Bun.file(`${import.meta.dir}/../tmp/${uuid}`).text();
  await $`kill ${pid}`;
  await $`rm -rf ${import.meta.dir}/../tmp`;
  await $`mkdir -p ${import.meta.dir}/../tmp`;
});

const corolla = new Corolla("http://127.0.0.1:50000", "/test");

const write01 = corolla.make_write_query<
  { vacation_spot: string }
>("write01");

const read01 = corolla.make_read_query<
  {},
  { id: string; vacation_spot: string }
>("read01");

test("write query", async () => {
  let res = await write01({ vacation_spot: "carrot" });
  expect(res.ok).toBeTrue;
  res = await write01({ vacation_spot: "tomato" });
  expect(res.ok).toBeTrue;
  res = await write01({ vacation_spot: "squash" });
  expect(res.ok).toBeTrue;
});

test("read query", async () => {
  const rows = await read01({});
  expect(rows !== null);
  if (rows !== null) {
    expect(rows[0].vacation_spot).toBe("carrot");
    expect(rows[1].vacation_spot).toBe("tomato");
    expect(rows[2].vacation_spot).toBe("squash");
  }
});
