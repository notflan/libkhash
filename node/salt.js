const ref = require('ref-napi');

/// Create a new salt from the string `buffer`.
function Salt(buffer, tag)
{
    this.tag = tag || '__SPECIFIC';
    if (buffer) {
	this.buffer = ref.allocCString(buffer);
    } else {
	this.buffer = null;
	this.tag = '__NONE';
    }
}

/// No salt.
Salt.None = new Salt(null, '__NONE');
/// A randomly generated salt.
Salt.Random = new Salt(null, '__RANDOM');
/// The static default salt.
Salt.Default = new Salt(null, '__DEFAULT');

module.exports = Salt;
