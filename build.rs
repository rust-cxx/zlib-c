fn main()
{
	cxx::Config::new()
		.define("GLEW_STATIC", "1")
		.build();
}