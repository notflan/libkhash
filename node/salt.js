const ref = require('ref');

function Salt(buffer, tag)
{
    this.tag = tag || '__SPECIFIC';
    if (buffer) {
	this.buffer = ref.allocCString(buffer);
    } else {
	this.buffer = null;
    }
}

Salt.None = new Salt(null, '__NONE');
Salt.Random = new Salt(null, '__RANDOM');
Salt.Default = new Salt(null, '__DEFAULT');

module.exports = Salt;
