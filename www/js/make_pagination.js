// функция которая формерует объект описания пагинации для последющего
// его засыла в шаблон pagination.hbs
define( function(require) {
    var make_pagination = function( current_page, pages_count, link_prefix ) {
        var pagination = { prefix: link_prefix };
        var minPage = current_page - 2;
        var maxPage = current_page + 2;
        var maxValidPage = pages_count - 1;

        minPage = Math.max( minPage, 0 );
        maxPage = Math.min( maxValidPage, maxPage );

        if ( current_page !== 0 ) {
            pagination.prev = String( current_page - 1 );
        }

        pagination.pages = [];
        for ( var i = minPage; i <= maxPage; ++i ) {
            var current = {
                name: i + 1
            };
            if ( i == current_page ) {
                current.active = true;
            } else {
                current.link = String( i );
            }
            pagination.pages.push( current );
        }

        if ( maxPage < maxValidPage ) {
            pagination.pages.push(
                { name: "..", disabled: true},
                { name: maxValidPage + 1, link: maxValidPage }
            );
        }

        if ( current_page < maxValidPage ) {
            pagination.next = String( current_page + 1 );
        }

        return pagination;
    }

    return make_pagination;
})
