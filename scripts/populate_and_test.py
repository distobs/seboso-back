from os import getenv
from dotenv import load_dotenv
import psycopg2
import requests

load_dotenv()

# HELPER


def req(method, rt, desc, jdata=None, jwt=None):
    endpoint = "http://localhost:3000" + rt
    fun = None
    match method:
        case "get":
            fun = requests.get
        case "post":
            fun = requests.post
        case "put":
            fun = requests.put
        case "delete":
            fun = requests.delete

    if jwt:
        r = fun(endpoint, json=jdata, headers={
                "Authorization": f"Bearer {jwt}"})
    else:
        r = fun(endpoint, json=jdata)

    j = r.json()
    print(j, r.status_code)
    print(desc)

    return j


def reqesp(method, rt, desc, jdata=None, jwt=None):
    endpoint = "http://localhost:3000" + rt
    fun = None
    match method:
        case "get":
            fun = requests.get
        case "post":
            fun = requests.post
        case "put":
            fun = requests.put
        case "delete":
            fun = requests.delete

    if jwt:
        r = fun(endpoint, json=jdata, headers={
                'Authorization': f"Bearer {jwt}"})
    else:
        r = fun(endpoint, json=jdata)

    print(r.text)
    print(desc)

# SAVE ADMIN JWT


POSTGRES_DB = getenv("POSTGRES_DB")
POSTGRES_USER = getenv("POSTGRES_USER")
POSTGRES_PASSWORD = getenv("POSTGRES_PASSWORD")

connection = psycopg2.connect(
    database=POSTGRES_DB,
    user=POSTGRES_USER,
    password=POSTGRES_PASSWORD,
    host="localhost",
    port="5432"
)

cursor = connection.cursor()

req("post", "/users", "Create admin", {
    "name": "Admin",
    "email": "admin@admin.com",
    "login": "admin",
    "password": "admin",
    "cell_number": "0000000000"
})

cursor.execute("UPDATE users SET is_admin = true WHERE login = 'admin';")

connection.commit()

j = req("post", "/users/login", "Login as admin", {
    "login": "admin",
    "password": "admin"
})

jwt_adm = j["message"]

# CREATE USERS

req("post", "/users", "Create user John 1", {
    "name": "John 1",
    "email": "john@john.com",
    "login": "john",
    "password": "1234",
    "cell_number": "123412342",
})

req("post", "/users", "Create user John 2", {
    "name": "John 2",
    "email": "john2@john.com",
    "login": "john2",
    "password": "1234",
    "cell_number": "123412343",
})

# READ USERS

req("get", "/users", "List users.")

req("get", "/users/2", "Get User with ID 2")

# LOGIN AND UPDATE USERS

j = req("post", "/users/login", "Login as john2", {
    "login": "john2",
    "password": "1234"
})

jwt = j["message"]

req("put", "/users/3", "Update John 2's name to Joseph 2", jdata={
    "name": "Joseph 2",
}, jwt=jwt)

req("get", "/users/3", "Confirm changes in Joseph 2.")

# DELETE USER

req("delete", "/users/3", "Delete Joseph 2", jwt=jwt)

req("get", "/users", "Confirm deletion")

# LOGIN AS JOHN 1

j = req("post", "/users/login", "Login as john1", {
    "login": "john",
    "password": "1234"
})

jwt = j["message"]

# CREATE STORES

sebos = [
    {
        "name": "Sebo 1",
        "cnpj": "1234",
        "street": "Rua da Goiaba",
        "number": 42,
        "city": "Lugarlândia",
        "state": "Localidade",
        "city_block": "ABCD",
        "cep": "0001"
    },
    {
        "name": "Sebo 2",
        "cnpj": "1235",
        "street": "Rua da Maçã",
        "number": 43,
        "city": "Lugarlândia",
        "state": "Localidade",
        "city_block": "ABCD",
        "cep": "0002"
    },
    {
        "name": "Sebo 3",
        "cnpj": "1236",
        "street": "Rua da Maçã",
        "number": 43,
        "city": "Lugarlândia",
        "state": "Localidade",
        "city_block": "ABCD",
        "cep": "0003"
    },
]

for sebo in sebos:
    req("post", "/stores", f"Criando sebo {sebo["name"]}", jdata=sebo, jwt=jwt)

# UPDATE STORES

sebos[0]["name"] = "Seboso 1"
sebos[0]["street"] = "Rua dos Penduricalhos"
sebos[1]["name"] = "Seboso 2123"
sebos[1]["street"] = "Av. Treze de Maio"
sebos[2]["number"] = 1234

for i in range(0, 3):
    req("put", f"/stores/{i + 1}",
        f"Atualizando sebo {sebos[i]["name"]}", jdata=sebos[i], jwt=jwt)

# READ STORES

req("get", "/stores", "Lendo sebos")

# ADD IN SOME BOOKS

books = [
    {
        "title": "Dom Casmurro",
        "author": "Machado de Assis",
        "description": "Descrição de Dom Casmurro",
        "published_at": "6767-06-07",
        "isbn_10_code": "1234567890",
        "isbn_13_code": "1234567890123",
        "cover_url": "https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEj8YTMEji396gKQDWxHzvuamDg61PP17lVX0Yibyul1qsI8Rv-CTplANZ0rdm-aG4vujaO3bypVIP8GM34dCCpdDP89hx_fiouDehWes1JbjclzJ_OfK8nX3uXlviCIqvOtWjCCCCPc0Mc/s400/Dom-Casmurro-Capa-do-Livro-1-machado_de_assis.jpg",
        "cover_type": "Capa comum",
        "edition": "1",
        "language": "Português",
        "genre": "Realismo",
        "publisher": "Livraria Garnier",
        "pages": 1000,
        "dimensions": "20x20x2 cm"
    },
    {
        "title": "Vision of Sir Launfal and Other Poems",
        "author": "James Russell Lowell",
        "description": "Number 30 in a series of literary pamphlets published monthly and available at the price of 15 cents per copy, or a yearly subscription (19 numbers) for $1.25",
        "published_at": "1887-11-01",
        "isbn_10_code": "1994200102016",
        "isbn_13_code": "1994200102019",
        "cover_url": "https://m.media-amazon.com/images/I/41h4rpDMclL._SY445_SX342_ML2_.jpg",
        "cover_type": "Paperback",
        "edition": "",
        "language": "eng",
        "genre": "poetry",
        "publisher": "Houghton, Mifflin and Company",
        "pages": 80,
        "dimensions": ""
    },
    {
        "title": "Fairy Tales: Dramolettes",
        "author": "Robert Walser",
        "description": "Fairy Tales gathers the unconventional verse dramolettes of the Swiss writer Robert Walser. Narrated in Walser's inimitable, playful language, these theatrical pieces overturn traditional notions of the fairy tale, transforming the Brothers Grimm into metatheater, even metareflections.\nSnow White forgives the evil queen for trying to kill her, Cinderella doubts her prince and enjoys being hated by her evil stepsisters; the Fairy Tale itself is a character who encourages her to stay within the confines of the story. Sleeping Beauty, the royal family, and its retainers are not happy about being woken from their sleep by an absurd, unpretentious, Walser-like hero. Mary and Joseph are taken aback by what lies in store for their baby Jesus.",
        "published_at": "2015-04-20",
        "isbn_10_code": "0811223981",
        "isbn_13_code": "9780811223980",
        "cover_url": "https://m.media-amazon.com/images/I/31i32F1fvGL._SY445_SX342_ML2_.jpg",
        "cover_type": "Paperback",
        "edition": "",
        "language": "",
        "genre": "drama",
        "publisher": "New Directions",
        "pages": 128,
        "dimensions": ""
    },
    {
        "title": "Growltiger's Last Stand and Other Poems",
        "author": "T.S. Eliot",
        "description": "Three poems describe the nighttime adventures of some rather special cats.",
        "published_at": "2008-07-12",
        "isbn_10_code": "0374428115",
        "isbn_13_code": "9780374428112",
        "cover_url": "https://m.media-amazon.com/images/I/A1F25L03iRL._SY342_.jpg",
        "cover_type": "Paperback",
        "edition": "",
        "language": "",
        "genre": "poetry",
        "publisher": "Farrar Straus Giroux",
        "pages": 50,
        "dimensions": ""
    },
    {
        "title": "The Cocktail Party",
        "author": "T.S. Eliot",
        "description": "A modern verse play about the search for meaning, in which a psychiatrist is the catalyst for the action. \"An authentic modern masterpiece\" (New York Post). \"Eliot really does portray real-seeming characters. He cuts down his poetic effects to the minimum, and then finally rewards us with most beautiful poetry\" (Stephen Spender).",
        "published_at": "1964-03-18",
        "isbn_10_code": "0156182890",
        "isbn_13_code": "9780156182898",
        "cover_url": "https://m.media-amazon.com/images/I/71fTEmAfhTL._SY342_.jpg",
        "cover_type": "Paperback",
        "edition": "",
        "language": "",
        "genre": "plays",
        "publisher": "Mariner Books",
        "pages": 190,
        "dimensions": ""
    },
    {
        "title": "Louder Than Everything You Love",
        "author": "Nicole Rollender",
        "description": "Louder Than Everything You Love is about transformation. The narrator in these poems is many: women who talk to the dead, women who mourn dead mothers and grandmothers, women suicides, women who've been raped/escaped rape, women who cradle premature babies, women who suffer depression, women who prepare the bodies of the dead, women who exist between their children's bodily needs (\"this body-psalm of need the only holiness I know\") and saints' incorruptible bodies.\nThese women also live inside themselves, contending with the wolves within, asking: \"How do I measure the body's gardens form within its bone fences?\" The dead, the living and the divine inhabit this collection - they're looking for kinship, remembrance, for some kind of communion. The poems in Louder Than Everything You Love are about the struggle of living in a body, being a parent, trying to find the balance between what our lives on earth mean/what it means to come to terms with dying.",
        "published_at": "2015-12-23",
        "isbn_10_code": "1942004192",
        "isbn_13_code": "9781942004196",
        "cover_url": "https://m.media-amazon.com/images/I/51fD0M24bVL._SY342_.jpg",
        "cover_type": "Paperback",
        "edition": "First",
        "language": "eng",
        "genre": "poetry",
        "publisher": "ELJ Publications",
        "pages": 118,
        "dimensions": ""
    },
    {
        "title": "Su Seviyesi",
        "author": "Joseph Brodsky",
        "description": "\"... Cunku gocup gideriz, guzellik ise kalicidir. Cunku gelecege yonelmis olmamiza ragmen guzellik ebedi simdiki zamandir.\"\nSu Seviyesi icin bir sehrin portresi denebilir. Ancak Nobel Odullu sair ve denemeci Brodsky'nin kalemi ve bakis acisi bu portreyi zeki, bilge, dokunakli ve tam anlamiyla zarif bir hale getiriyor. Brodsky bir flanor gibi Venedik'in caddelerinde, sokaklarinda, kanallarinda, su basmis kiliselerinde ve dehlizlere acilan eski yapilarinda dolaniyor; insanlari, mutfagi, tarihi, hatta giyim kusami hakkinda birbirinden ilginc bilgiler veriyor, tanik oldugu tuhaf anekdotlar anlatiyor. Bu su sehrinden unutulmaz kis manzaralari, sularin yukselmesiyle gelen olaganustu guzellikte goruntuler resmediyor. Boylece sehrin kiside uyandirdigi tuhaf, tekinsiz ama alabildigine siirsel imgelerin pesine dusup insanin suyla, dolayli olarak da onun yansimasi zaman ve guzellik kavramiyla ikircikli iliskisini irdeliyor - ama ancak bir sairin, hem de buyuk bir sairin yapabilecegi sekilde. Su Seviyesi, Venedik'i sularin kusatmasi gibi zamanin da insani kusattigi bir gerceklik resmediyor sakinmadan -- buna karsin \"guzellik\" veya \"sevgi\" gibi, zamani asmanin yollarini isaret ederek...",
        "published_at": "2017-05-26",
        "isbn_10_code": "",
        "isbn_13_code": "9786051851471",
        "cover_url": "https://m.media-amazon.com/images/I/711XgRsQr+L._SY342_.jpg",
        "cover_type": "Paperback",
        "edition": "",
        "language": "tur",
        "genre": "non-fiction",
        "publisher": "Everest Yayinlari",
        "pages": 128,
        "dimensions": ""
    },
    {
        "title": "Into Temptation",
        "author": "Sophia Blackwell",
        "description": "Into Temptation is the debut collection of poems from Sophia Blackwell, a regular on the UK poetry scene who has been holding crowds spellbound for over five years in bars, nightclubs, and festival tents.\nSophia takes us through love in all its guises - burning obsession, one-night stands that last too long, domestic bliss, and the insecurities in even the most loving relationships. Here also is a series of elegies for the everyday - the quiet, mysterious pleasures of Tube journeys, cats, and spaghetti sauce.\nInto Temptation is all lipstick, corsets and hedonistic jazz-fuelled rhythms, raging against and revelling in life.",
        "published_at": "2009-01-01",
        "isbn_10_code": "1994200102017",
        "isbn_13_code": "1994200102020",
        "cover_url": "https://m.media-amazon.com/images/I/41N-b+q9m4L._SY342_.jpg",
        "cover_type": "Paperback",
        "edition": "",
        "language": "eng",
        "genre": "poetry",
        "publisher": "Tollington Press",
        "pages": 80,
        "dimensions": ""
    },
    {
        "title": "Dark Sons",
        "author": "Nikki Grimes",
        "description": "",
        "published_at": "",
        "isbn_10_code": "1994200102018",
        "isbn_13_code": "1994200102021",
        "cover_url": "https://m.media-amazon.com/images/I/91h6kTYbsUL._AC_UY218_.jpg",
        "cover_type": "",
        "edition": "",
        "language": "",
        "genre": "poetry",
        "publisher": "",
        "pages": 50,
        "dimensions": ""
    },
    {
        "title": "دریا پری، کاکل زری",
        "author": "Goli Taraghi Farzan",
        "description": "",
        "published_at": "2005-01-01",
        "isbn_10_code": "9646138675",
        "isbn_13_code": "",
        "cover_url": "https://i.ebayimg.com/images/g/RpYAAeSwK2hn~Cx8/s-l500.jpg",
        "cover_type": "Paperback",
        "edition": "",
        "language": "per",
        "genre": "poetry",
        "publisher": "frzn",
        "pages": 72,
        "dimensions": ""
    },
    {
        "title": "Borderlands/La Frontera",
        "author": "Gloria Anzaldúa",
        "description": "First published in 1987, \"Borderlands\" has become a classic in Chicano border studies, feminist theory, gay and lesbian studies, and cultural studies. Anzaldua, a Chicana native of Texas, explores in prose and poetry the murky, precarious existence of those living on the frontier between cultures and languages. Writing in a lyrical mixture of Spanish and English that is her unique heritage, she meditates on the condition of Chicanos in Anglo culture, women in Hispanic culture, and lesbians in the straight world. Her essays and poems range over broad territory, moving from the plight of undocumented migrant workers to memories of her grandmother, from Aztec religion to the agony of writing. Anzaldua is a rebellious and willful talent who recognizes that life on the border,\"life in the shadows,\"is vital territory for both literature and civilization. Venting her anger on all oppressors of people who are culturally or sexually different, the author has produced a powerful document that belongs in all collections with emphasis on Hispanic American or feminist issues.",
        "published_at": "1987-12-12",
        "isbn_10_code": "1879960125",
        "isbn_13_code": "9781879960121",
        "cover_url": "https://m.media-amazon.com/images/G/01/apparel/rcxgs/tile._CB483369110_.gif",
        "cover_type": "Paperback",
        "edition": "",
        "language": "",
        "genre": "non-fiction",
        "publisher": "Aunt Lute Books",
        "pages": 50,
        "dimensions": ""
    },
    {
        "title": "Naked Soul: The Erotic Love Poems",
        "author": "Salil Jha",
        "description": "\"Erotic poetry that evokes feelings of joy, happiness, and an overall celebration of the arts of physical and romantic love.\" - KIRKUS REVIEW\n\"Naked Soul: The Erotic Love Poems\" is an extraordinary storytelling in the form of erotic love poetry, speaking directly to the reader's heart through sensations that course throughout the body.\nThis powerful collection of erotic and sensual love poems celebrates the erotic spirit in all its forms -- from intense passionate sexual desire to seductive victory.\nThere are love poems for every mood and sentimental feeling, for every phase of love you are experiencing whether you are with a partner or not. Read it slowly. Read a poem at a time, or two-or all at once-but give it time to sink into your heart. Read them again. Visualize. Let the poem show you what may be lying dormant in your own heart.\nAny poetry lover who loves deep symbolism, storytelling and musing over deep verses will find this book very touching. No matter which phase of love you are growing in currently, this book will serve to sail you further towards the endless ocean of love.\n\"The book was as good a collection of modern erotic poems (by a single author) as may be found.\" - PoemShape\n\"You will feel something when you read this book. This [book] belongs to the author, [but] also to me, you and everyone.\" - MyBookShelfIsYours\n\"Highly Recommended.\" - KHouse",
        "published_at": "2015-01-09",
        "isbn_10_code": "0692265295",
        "isbn_13_code": "9780692265291",
        "cover_url": "https://m.media-amazon.com/images/I/61wnApTfa+L._SY342_.jpg",
        "cover_type": "Paperback",
        "edition": "",
        "language": "eng",
        "genre": "poetry",
        "publisher": "Naked Soul Press",
        "pages": 172,
        "dimensions": ""
    },
    {
        "title": "Geografías",
        "author": "Mario Benedetti",
        "description": "",
        "published_at": "",
        "isbn_10_code": "9681911288",
        "isbn_13_code": "9789681911287",
        "cover_url": "https://m.media-amazon.com/images/G/01/apparel/rcxgs/tile._CB483369110_.gif",
        "cover_type": "",
        "edition": "",
        "language": "",
        "genre": "short-stories",
        "publisher": "",
        "pages": 50,
        "dimensions": ""
    },
    {
        "title": "Black Liquor: Poems",
        "author": "Dennis E. Bolen",
        "description": "Dennis E. Bolen's forthcoming volume of poetry, \"Black Liquor,\" continues his exploration of modern disconnection and the disparate paths taken by those railing against the austere landscape of their lives. Imbued with lyrical evocations of lost childhood, mature love and deep friendship contrasted against brutal depictions of grueling labour, industrial mishap, historical misfortune and often hilarious disappointment, \"Black Liquor\" progresses to an appreciation of being alive, against the odds. Bolen writes in the pacey cadences of contemporary speech, tough and tender. His quirky use of metaphorical story charged with biting imagery makes these deeply autobiographical poems an exhilaration. As in his previous writings, five novels and two collections of short fiction-among them \"Stupid Crimes\" (originally published by Anvil Press), \"Stand in Hell\" (Random House), \"Kaspoit!\" (Anvil Press), and \"Anticipated Results\" (Arsenal Pulp Press)-this new book explores the varieties of disaffection, this time in poetry and this time as remembrance of things past.",
        "published_at": "2013-09-01",
        "isbn_10_code": "1927575249",
        "isbn_13_code": "9781927575246",
        "cover_url": "https://m.media-amazon.com/images/I/61ATSXO9y9L._SY342_.jpg",
        "cover_type": "Paperback",
        "edition": "",
        "language": "",
        "genre": "poetry",
        "publisher": "Caitlin Press",
        "pages": 128,
        "dimensions": ""
    },
    {
        "title": "Cornflakes: Poems",
        "author": "James Stevenson",
        "description": "Want to savor something really delicious? Try something snappy? Give yourself a treat? Open this book!",
        "published_at": "2000-03-01",
        "isbn_10_code": "0688167187",
        "isbn_13_code": "9780688167189",
        "cover_url": "https://m.media-amazon.com/images/I/51EB0O1v+ML._SY342_.jpg",
        "cover_type": "Hardcover",
        "edition": "",
        "language": "",
        "genre": "poetry",
        "publisher": "Greenwillow Books",
        "pages": 56,
        "dimensions": ""
    },
    {
        "title": "https://m.media-amazon.com/images/I/81TooDLMM7L._SY342_.jpg",
        "author": "Michael Ryan",
        "description": "",
        "published_at": "1990-08-01",
        "isbn_10_code": "0140586202",
        "isbn_13_code": "9780140586206",
        "cover_url": "",
        "cover_type": "Paperback",
        "edition": "",
        "language": "",
        "genre": "poetry",
        "publisher": "Puffin",
        "pages": 96,
        "dimensions": ""
    },
    {
        "title": "The More Loving One",
        "author": "Kiki Denis",
        "description": "Free online poetry.\nFrom Random House's Boldtype.\nExcerpted from The Voice of the Poet: W.H. Auden by W.H. Auden.",
        "published_at": "2000-10-01",
        "isbn_10_code": "068819027187234",
        "isbn_13_code": "068819027187235",
        "cover_url": "https://m.media-amazon.com/images/G/01/apparel/rcxgs/tile._CB483369110_.gif",
        "cover_type": "ebook",
        "edition": "",
        "language": "eng",
        "genre": "poetry",
        "publisher": "http://www.randomhouse.com/boldtype/",
        "pages": 2,
        "dimensions": ""
    },
    {
        "title": "Inherit the Wind",
        "author": "Jerome Lawrence",
        "description": "A meaningful play based on the Scopes Monkey Trial of 1925, in which a Tennessee teacher was tried for teaching evolution. The accused was a slight, frightened man who'd deliberately broken the law. His trial was a Roman circus, the chief gladiators being the two great legal giants of the century. Locked in mortal combat, they bellowed & roared imprecations & abuse. The spectators sat uneasily in the sweltering heat with murder in their hearts, barely restraining themselves. America's freedom was at stake.",
        "published_at": "",
        "isbn_10_code": "0553269151",
        "isbn_13_code": "9780553269154",
        "cover_url": "https://m.media-amazon.com/images/I/613tuP23wSL._SY342_.jpg",
        "cover_type": "",
        "edition": "",
        "language": "en-US",
        "genre": "rory-gilmore-reading-challenge",
        "publisher": "",
        "pages": 50,
        "dimensions": ""
    },
    {
        "title": "Ariel",
        "author": "4379",
        "description": "\"In these poems...Sylvia Plath becomes herself, becomes something imaginary, newly, wildly and subtly created.\"\n-- From the Introduction by Robert Lowell",
        "published_at": "2011-05-01",
        "isbn_10_code": "2070441490",
        "isbn_13_code": "9782070441495",
        "cover_url": "https://images.gr-assets.com/books/1338943582m/11739266.jpg",
        "cover_type": "Mass Market Paperback",
        "edition": "",
        "language": "fre",
        "genre": "poetry",
        "publisher": "Gallimard",
        "pages": 50,
        "dimensions": ""
    },
]

for book in books:
    req("post", "/books",
        f"Criando livro {book['title']}", jdata=book, jwt=jwt_adm)

# ADD SOME BOOKS TO STORES' CATALOGS

catalogs = [
    {
        "store_id": 1,
        "book_id": 1,
        "price": 10.99,
        "quantity": 5,
        "description": "só o filé"
    },
    {
        "store_id": 1,
        "book_id": 2,
        "price": 12.99,
        "quantity": 3,
        "description": "todo lascado"
    },
    {
        "store_id": 2,
        "book_id": 2,
        "price": 11.99,
        "quantity": 4,
        "description": "bom"
    },
    {
        "store_id": 2,
        "book_id": 3,
        "price": 15.99,
        "quantity": 2,
        "description": "ótimo"
    }
]

# John 1 owns everything anyway.
for catalog in catalogs:
    req("post", "/catalog",
        f"Adicionando livro {catalog['book_id']} ao catálogo da loja {catalog['store_id']}", jdata=catalog, jwt=jwt)

# Create a new guy

req("post", "/users", "Create user John 3", {
    "name": "John 3",
    "email": "john3@john.com",
    "login": "john3",
    "password": "1234",
    "cell_number": "123412343",
})

j = req("post", "/users/login", "Login as John 3", {
    "login": "john3",
    "password": "1234"
})

jwt2 = j["message"]

# Make the new guy a worker

req("post", "/userstore", "Make John 3 a worker in sebo Seboso 1", jdata={
    "store_id": 1,
    "user_id": 4,
    "role": "worker"
}, jwt=jwt)

# Do worker stuff

req("put", "/catalog?store_id=1&book_id=1", "Update book 1 in catalog of Seboso 1 as John 3", jdata={
    "price": 100.50,
    "quantity": 1234,
}, jwt=jwt2)

req("delete", "/catalog?store_id=1&book_id=2",
    "Delete book 2 from catalog of Seboso 1 as John 3", jwt=jwt2)

# Fire John 3

req("delete", "/userstore", "Fire John 3", jdata={
    "user_id": 4,
    "store_id": 1,
    "role": "worker",
}, jwt=jwt)
