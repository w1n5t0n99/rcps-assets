ALTER TABLE asset_types
ADD full_search tsvector
GENERATED ALWAYS AS (
    setweight(to_tsvector('english', coalesce(brand, '')), 'A') || ' ' || 
    setweight(to_tsvector('english', coalesce(brand, '')), 'B') || ' ' || 
    setweight(to_tsvector('english', coalesce(description, '')), 'C') || ' ' || 
	setweight(to_tsvector('simple', coalesce(cost, '')), 'D') :: tsvector
) STORED