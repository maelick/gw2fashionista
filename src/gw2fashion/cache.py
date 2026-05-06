import logging

_MAX_BATCH_SIZE = 200

class Cache:
    def __init__(self, api):
        self.items = ObjectCache(api.items, 'items')
        self.skins = ObjectCache(api.skins, 'skins')
        self.colors = ObjectCache(api.colors, 'colors')
        self.outfits = ObjectCache(api.outfits, 'outfits')


class ObjectCache:
    ids: list[int]
    objects: dict[int, dict]

    def __init__(self, api, obj_type: str, batch_size=_MAX_BATCH_SIZE):
        self.api = api
        self.obj_type = obj_type
        self.batch_size = max(min(batch_size, _MAX_BATCH_SIZE), 1)
        self.clear()

    def clear(self):
        self.ids = []
        self.objects = {}

    def get_all(self) -> list[dict]:
        ids = self.get_ids()
        return self.get_many(ids)

    def get_ids(self) -> list[int]:
        if not self.ids:
            logging.info(f'Retrieving all {self.obj_type} ids')
            self.ids = self.api.get()
        return self.ids

    def get(self, id) -> dict:
        if id not in self.objects:
            logging.info(f'Retrieving {self.obj_type} with id {id}')
            self.objects[id] = self.api.get(id=id)
        return self.objects[id]

    def get_many(self, ids: list[int]) -> list[dict]:
        missing_ids = [id for id in ids if id not in self.objects]
        if len(missing_ids):
            self.fetch_missings(missing_ids)
        return [self.objects[id] for id in ids]

    def fetch_missings(self, ids: list[int]):
        n = len(ids)
        logging.info(f'Retrieving {n} missings {self.obj_type}')
        # TODO would be better with Python 3.12 itertools.batched function
        for i in range(0, n, 200):
            batch_ids = ids[i:min(i + 200, len(ids))]
            batch_objects = self.api.get(ids=batch_ids)
            for obj in batch_objects:
                self.objects[obj['id']] = obj
            logging.info(f'Retrieved {len(batch_ids)} missings {self.obj_type}, {max(0, n-i-200)} remaining')

# TODO use redis cache?
