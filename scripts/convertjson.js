const fs = require("fs");
const readline = require("readline");

const inputFile = "/home/dedsec/projects/seboso-back/scripts/books.json";
const outputFile = "books-converted.json";

function buildDate(year, month, day) {
  if (!year) return null;

  const y = String(year).padStart(4, "0");
  const m = String(month || 1).padStart(2, "0");
  const d = String(day || 1).padStart(2, "0");

  return `${y}-${m}-${d}`;
}

function extractGenre(popularShelves) {
  if (!Array.isArray(popularShelves)) return null;

  const ignored = new Set([
    "to-read",
    "currently-reading",
    "owned",
    "favorites",
    "default",
    "library",
    "books-i-own",
    "read"
  ]);

  const shelf = popularShelves.find(
    s => s?.name && !ignored.has(s.name.toLowerCase())
  );

  return shelf?.name || null;
}

function convertBook(book) {
  return {
    title: book.title || null,

    author:
      Array.isArray(book.authors) && book.authors.length
        ? (book.authors[0].name || book.authors[0].author_id || null)
        : null,

    description: book.description || null,

    published_at: buildDate(
      book.publication_year,
      book.publication_month,
      book.publication_day
    ),

    isbn_10_code: book.isbn || null,

    isbn_13_code: book.isbn13 || null,

    cover_url: book.image_url || null,

    cover_type: book.format || null,

    edition: book.edition_information || null,

    language: book.language_code || null,

    genre: extractGenre(book.popular_shelves),

    publisher: book.publisher || null,

    pages: Number(book.num_pages) || null,

    dimensions: null
  };
}

async function convertFile() {
  const results = [];

  const rl = readline.createInterface({
    input: fs.createReadStream(inputFile),
    crlfDelay: Infinity
  });

  for await (const line of rl) {
    if (!line.trim()) continue;

    try {
      const book = JSON.parse(line);
      results.push(convertBook(book));
    } catch (err) {
      console.error("Erro ao processar linha:", err.message);
    }
  }

  fs.writeFileSync(
    outputFile,
    JSON.stringify(results, null, 2),
    "utf8"
  );

  console.log(
    `Conversão concluída. ${results.length} registros salvos em ${outputFile}`
  );
}

convertFile();