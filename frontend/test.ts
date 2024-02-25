async function make_req() {
	console.log(await fetch("https://www.catlin.edu/"))
    let test_res = await fetch("http://localhost:3021/dummyData")
    console.log(test_res)
}

make_req()
