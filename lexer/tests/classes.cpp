class ExampleClass {
    public:
        string randomProperty;
        //std::cout also parses correctly
        void printname() { std::cout << "Some string" << randomProperty; }
};