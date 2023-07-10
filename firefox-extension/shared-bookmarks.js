try {
    var b = typeof browser === undefined ? chrome : browser;
} catch (e) {
    var b = chrome;
}


b.pageAction.onClicked.addListener(async () => {
    let tab = await browser.tabs.query({active: true, currentWindow: true});

    if (tab[0] === undefined) {
        return;
    }
    tab = tab[0];

    fetch("http://localhost:8000/api/addPost", {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({
            url: tab.url,
            title: tab.title
            // todo add tab.faviconUrl
        })
    });
});

