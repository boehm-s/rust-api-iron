function recup_info () {

    $.ajax({
	url: 'http://localhost:3000/api/1/factories/get_types',
	type: 'GET'
    }).done(function(data) {
	    document.getElementById('usine').innerHTML =data.map(function(el) {
		return [
		    '<div class="factory">',
		    '<h3 class="factory_name">', el.name ,'</h3>',
		    '<img class="factory_img" alt="factory" src="', el.image_url ,'">',
		    '<button data-id="', el.id ,'" class="factory_add"> ADD </button>',
		    '</div>'
		].join('');
	    }).join('');


	    Array.from(document.getElementsByClassName('factory_add')).forEach(function(el){
		el.onclick = function(e) {
		    e.preventDefault();

		    document.getElementById('factory_number').innerHTML = parseInt(document.getElementById('factory_number').innerHTML) + 1;

		    $.ajax({
			url: 'http://localhost:3000/api/1/factories/create',
			type: 'POST',
			data: JSON.stringify({type: 1, user_id: parseInt(localStorage.getItem('userId')), planet_id: 1})
		    }).done(function(data) {
			data = JSON.parse(data);
			document.getElementById('factories_display').innerHTML +=
			    ['<div class="factory_">', 'Factory ', data.id, '</div>'].join('');
		    });
		};
	    });

    }).fail(function(e) {
	console.log("error", e);
    });

    $.ajax({
	url: 'http://localhost:3000/api/1/factories/get/' + localStorage.getItem('userId'),
	type: 'GET'
    }).done(function(data) {
	document.getElementById('factory_number').innerHTML = data.length;
	data.forEach(function(obj) {
	    document.getElementById('factories_display').innerHTML +=
		['<div class="factory_">', 'Factory ', obj.id, '</div>'].join('');
	});
    });







    // $.ajax({
    // 	url: 'http://163.5.245.219:3000/api/1/crystalmines',
    // 	type: 'GET',
    // 	xhrFields: { withCredentials: true },
    // 	data: JSON.stringify({
    // 		"username": localStorage.getItem("login"),
    // 		"password": localStorage.getItem("pass")
    // 	}),
    // })
    // .done(function(data) {
    // 	$($($('#accueil div')[1]).children('span')[0]).empty().text(data.level);
    // 	$($($('#accueil div')[1]).children('span')[1]).empty().text(data.production);
    // })
    // .fail(function() {
    // 	console.log("error");
    // });

    // $.ajax({
    // 	url: 'http://163.5.245.219:3000/api/1/metalmines',
    // 	type: 'GET',
    // 	xhrFields: { withCredentials: true },
    // 	data: JSON.stringify({
    // 		"username": localStorage.getItem("login"),
    // 		"password": localStorage.getItem("pass")
    // 	}),
    // })
    // .done(function(data) {
    // 	$($($('#accueil div')[0]).children('span')[0]).empty().text(data.level);
    // 	$($($('#accueil div')[0]).children('span')[1]).empty().text(data.production);
    // })
    // .fail(function() {
    // 	console.log("error");
    // });

    $.ajax({
	url: 'http://localhost:3000/api/1/factories/get/' + localStorage.getItem('userId'),
	type: 'GET'
    })
	.done(function(data) {
	    console.log(data);
	})
	.fail(function() {
	    console.log("error");
	});

    // $.ajax({
    // 	url: 'http://163.5.245.219:3000/api/1/droids',
    // 	type: 'GET',
    // 	xhrFields: { withCredentials: true },
    // 	data: JSON.stringify({
    // 		"username": localStorage.getItem("login"),
    // 		"password": localStorage.getItem("pass")
    // 	}),
    // })
    // .done(function(data) {
    // 	var nb = 0;
    // 	$.each(data.troops, function(index, val) {
    // 		nb += val;
    // 	});
    // 	$($($('#accueil div')[3]).children('span')[0]).empty().text(nb);
    // })
    // .fail(function() {
    // 	console.log("error");
    // });

}
