using System;
using System.Collections.Generic;

namespace Common
{
    public class DefaultDictionary<TKey, TValue> : Dictionary<TKey, TValue>
    {
        Func<TValue> _init;

        public DefaultDictionary(Func<TValue> init)
        {
            _init = init;
        }

        public new TValue this[TKey k]
        {
            get
            {
                if (!ContainsKey(k))
                    Add(k, _init());
                return base[k];
            }
            set => base[k] = value;
        }
    }

    public struct Position
    {
        public readonly int x, y;

        public Position(int x, int y)
        {
            this.x = x;
            this.y = y;
        }

        public Position Add(Position o)
        {
            return new Position(this.x + o.x, this.y + o.y);
        }

        public override int GetHashCode()
        {
            return (this.x, this.y).GetHashCode();
        }


        public override bool Equals(Object obj)
        {
            if ((obj == null) || !this.GetType().Equals(obj.GetType()))
            {
                return false;
            }
            else
            {
                Position p = (Position)obj;
                return (x == p.x) && (y == p.y);
            }
        }

        public bool Equals(Position o)
        {
            return this.x == o.x && this.y == o.y;
        }
    }
}
