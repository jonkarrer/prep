CREATE TABLE tags (
    tag_id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    recipe_id CHAR(36) NOT NULL,
    user_id CHAR(36) NOT NULL,
    tag_name VARCHAR(255) NOT NULL
)