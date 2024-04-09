import { Corolla } from "../js_api/index";
import { beforeAll, afterAll, expect, test } from "bun:test";

let proc;
beforeAll(() => {
	proc = Bun.spawn(["cargo", "run", "--", "-s", "examples/example_spec.json", "-d", "tmp/api_test.sqlite3", "-r", "/test"]);
});
afterAll(() => {
	proc.kill();
});

const corolla = new Corolla("http://127.0.0.1:50000", "/test");

const write01 = corolla.make_write_query<
	{ c: string }
>("write01");

const read01 = corolla.make_read_query<
	{},
	{ id: string; c: string }
>("read01");

test("write query", async () => {
	let res = await write01({ c: "carrot" });
	expect(res.status).toBe(200);
	res = await write01({ c: "tomato" });
	expect(res.status).toBe(200);
	res = await write01({ c: "squash" });
	expect(res.status).toBe(200);
});

test("read query", async () => {
	const rows = await read01({});
	expect(
		JSON.stringify(rows) ===
		JSON.stringify([{ c: "carrot" }, { c: "tomato" }, { c: "squash" }]),
	);
});
