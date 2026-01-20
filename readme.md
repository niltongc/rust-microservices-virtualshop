Migration:

migration init:
```sh
sea-orm-cli migrate init
```

migration schema:
```sh
sea-orm-cli migrate generate create_product_schema
```

remove in lib.rs old mod and ref
set drives on cargo.toml migration

migration schema:
```sh
sea-orm-cli migrate up
```

check db:

```sql
SHOW COLUMNS FROM products;

SHOW COLUMNS FROM categories;
```

```sql
-- Insert category
INSERT INTO categories (name) VALUES ('IT');

-- Insert product
INSERT INTO products
(name, price, description, stock, image_url, category_id)
VALUES ('Mouse', 99.90, 'Mouse gamer', 10, 'mouse.png', 1);

```

generate entity

```sh
sea-orm-cli generate entity -u mysql://root:YourPass@127.0.0.1:3306/vshop -o src/entity
```