import nexmark

def test_generate_ten_events():
    config = nexmark.Config()
    generator = nexmark.EventGenerator(config)
    events = []
    for i, event in enumerate(generator):
        if i >= 10:
            break
        events.append(event)
    print(events)

if __name__ == "__main__":
    test_generate_ten_events()