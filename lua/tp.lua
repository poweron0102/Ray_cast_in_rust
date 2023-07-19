print(TilePosition.locateX, TilePosition.locateY)
print(get_tile(Player.locateX, Player.locateY).render)
test_tile = get_tile(Player.locateX, Player.locateY)
--test_tile.render = true
--test_tile.is_wall = true
test_tile.color.r = 1
test_tile.color.a = 1
set_tile(Player.locateX, Player.locateY, test_tile)
Player.locateX = 900
