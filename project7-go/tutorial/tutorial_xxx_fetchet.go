package tutorial

import (
	"fmt"
	"sync"
)

type Fetcher interface {
	// Fetch returns the body of URL and
	// a slice of URLs found on that page.
	Fetch(url string) (body string, urls []string, err error)
}

type History struct {
	history map[string]bool
	mux     sync.Mutex
}

var history = History{history: make(map[string]bool)}

func (h *History) isFetched(url string) bool {
	h.mux.Lock()
	defer h.mux.Unlock()

	_, ok := h.history[url]
	if ok {
		return ok
	} else {
		h.history[url] = true
	}
	return false
}

// Crawl uses fetcher to recursively crawl
// pages starting with url, to a maximum of depth. // ,
func Crawl(url string, depth int, fetcher Fetcher, wg *sync.WaitGroup) {

	// TODO: Fetch URLs in parallel.
	// This implementation doesn't do either:

	defer wg.Done()
	if depth <= 0 || history.isFetched(url) {
		return
	}

	body, urls, err := fetcher.Fetch(url)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Printf("found: %s %q\n", url, body)
	for _, u := range urls {
		wg.Add(1)
		go Crawl(u, depth-1, fetcher, wg)
	}

	return
}

func syncCrawl(url string, depth int, fetcher Fetcher) {
	var wg sync.WaitGroup
	wg.Add(1)
	go Crawl(url, depth, fetcher, &wg)
	wg.Wait()
}

// fakeFetcher is Fetcher that returns canned results.
type fakeFetcher map[string]*fakeResult

type fakeResult struct {
	body string
	urls []string
}

func (f fakeFetcher) Fetch(url string) (string, []string, error) {
	if res, ok := f[url]; ok {
		return res.body, res.urls, nil
	}
	return "", nil, fmt.Errorf("not found: %s", url)
}

// fetcher is a populated fakeFetcher.
var fetcher = fakeFetcher{
	"https://golang.org/": &fakeResult{
		"The Go Programming Language",
		[]string{
			"https://golang.org/pkg/",
			"https://golang.org/cmd/",
		},
	},
	"https://golang.org/pkg/": &fakeResult{
		"Packages",
		[]string{
			"https://golang.org/",
			"https://golang.org/cmd/",
			"https://golang.org/pkg/fmt/",
			"https://golang.org/pkg/os/",
		},
	},
	"https://golang.org/pkg/fmt/": &fakeResult{
		"Package fmt",
		[]string{
			"https://golang.org/",
			"https://golang.org/pkg/",
		},
	},
	"https://golang.org/pkg/os/": &fakeResult{
		"Package os",
		[]string{
			"https://golang.org/",
			"https://golang.org/pkg/",
		},
	},
}

func ExecuteCrawler() {
	syncCrawl("https://golang.org/", 4, fetcher)
}
