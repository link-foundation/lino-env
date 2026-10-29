import { test, assert } from 'test-anywhere';
import { Link, Parser } from 'links-notation';

// Exercise the dependency's public API independently of the .lenv reader.
// .lenv values retain their original quoting and whitespace conventions.
for (const [name, ending] of [
  ['LF', '\n'],
  ['CRLF', '\r\n'],
]) {
  test(`Links Notation parser/formatter roundtrip with ${name}`, () => {
    const parser = new Parser();
    const links = parser.parse(
      ['LAZADA_MARKET: vn', 'CITY: "Nha Trang"', 'ENABLED: true'].join(ending)
    );
    assert.deepEqual(
      links.map((link) => [link.id, link.values.map((value) => value.id)]),
      [
        ['LAZADA_MARKET', ['vn']],
        ['CITY', ['Nha Trang']],
        ['ENABLED', ['true']],
      ]
    );
    const formatted = links.map((link) => link.toString()).join(ending);
    const reparsed = parser.parse(formatted);
    assert.equal(reparsed.length, links.length);
    for (let i = 0; i < links.length; i++) {
      assert.ok(reparsed[i].equals(links[i]));
    }
    assert.ok(links[1].equals(new Link('CITY', [new Link('Nha Trang')])));
  });
}
