def open_browser():
    import webbrowser
    webbrowser.open('127.0.0.1:8080')

def main():
    for i in range(10):
        open_browser()

main()