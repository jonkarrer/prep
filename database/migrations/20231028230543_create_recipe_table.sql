CREATE TABLE recipes (
    row_id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    recipe_id CHAR(36) UNIQUE NOT NULL,
    user_id CHAR(36) NOT NULL,
    recipe_title VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    favorite BOOLEAN DEFAULT FALSE,
    servings FLOAT
)