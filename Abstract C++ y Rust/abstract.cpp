
#include <iostream>
#define Krusty

using namespace std;


class Widget {
 public:
  virtual void draw() = 0;
};

/**
 * Concrete product family 1.
 */
class Kangreburger : public Widget {
 public:
  void draw() { cout << "Kangreburger\n"; }
};
class PlantonBurger: public Widget {
 public:
  void draw() { cout << "PlantonBurger\n"; }
};

/**
 * Concrete product family 2.
 */
class KrustyKrab : public Widget {
 public:
  void draw() { cout << "KrustyKrab\n"; }
};
class ChumBucket : public Widget {
 public:
  void draw() { cout << "ChumBucket\n"; }
};

/**
 * Abstract factory defines methods to create all
 * related products.
 */
class Factory {
 public:
  virtual Widget *create_burger() = 0;
  virtual Widget *create_restaurante() = 0;
};

/**
 * Each concrete factory corresponds to one product
 * family. It creates all possible products of
 * one kind.
 */
class KrustyFactory : public Factory {
 public:
  Widget *create_burger() {
    return new Kangreburger;
  }
  Widget *create_restaurante() {
    return new PlantonBurger;
  }
};

/**
 * Concrete factory creates concrete products, but
 * returns them as abstract.
 */
class ChumFactory : public Factory {
 public:
  Widget *create_burger() {
    return new KrustyKrab;
  }
  Widget *create_restaurante() {
    return new ChumBucket;
  }
};



class Client {
 private:
  Factory *factory;

 public:
  Client(Factory *f) {
    factory = f;
  }

  void draw() {
    Widget *w = factory->create_burger();
    w->draw();
    display_window_one();
    display_window_two();
  }

  void display_window_one() {
    Widget *w[] = {
            factory->create_burger(),
            factory->create_restaurante()
    };
    w[0]->draw();
    w[1]->draw();
  }

  void display_window_two() {
    Widget *w[] = {
            factory->create_restaurante(),
        factory->create_burger()
    };
    w[0]->draw();
    w[1]->draw();
  }
};

/*

int main() {
    Factory *factory;
#ifdef Krusty
    factory = new KrustyFactory;
#else // Chum
    factory = new ChumFactory;
#endif

    Client *c = new Client(factory);
    c->draw();
}

*/